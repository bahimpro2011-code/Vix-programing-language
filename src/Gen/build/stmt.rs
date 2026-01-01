use crate::import::*;

impl Codegen {
    pub fn codegen_struct_definition(&mut self, struct_def: &StructDef) -> Result<(), ()>  {
        let loc = self.default_location();

        if self.structs.contains_key(&struct_def.name) {
            self.diagnostics.error(
                "DuplicateStruct",
                &format!("Struct '{}' is already defined", struct_def.name),
                ErrorContext {
                    primary_location: loc.clone(),
                    secondary_locations: vec![],
                    help_message: Some("Each struct must have a unique name.".to_string()),
                    suggestions: vec![
                        format!("Rename this struct to something else"),
                        "Check for duplicate struct definitions".to_string(),
                    ],
                }
            );
        }
        
        let mut struct_code = format!("typedef struct {} {{\n", struct_def.name);
        let mut fields_info = Vec::new();
        
        for field in &struct_def.fields {
            if matches!(field.ty, Type::Void) {
                self.diagnostics.error(
                    "VoidField",
                    &format!("Field '{}' in struct '{}' cannot be void", field.name, struct_def.name),
                    ErrorContext {
                        primary_location: loc.clone(),
                        secondary_locations: vec![],
                        help_message: Some("Struct fields must have concrete types.".to_string()),
                        suggestions: vec![
                            format!("Change field '{}' to a concrete type", field.name),
                        ],
                    }
                );
            }
            
            let c_type = field.ty.to_c_type(&self.arch);
            struct_code.push_str(&format!("    {} {};\n", c_type, field.name));
            fields_info.push((field.name.clone(), field.ty.clone(), field.is_public));
        }
        
        struct_code.push_str(&format!("}} {};\n\n", struct_def.name));
        
        self.structs.insert(
            struct_def.name.clone(),
            StructInfo {
                fields: fields_info,
                llvm_type: struct_def.name.clone(),
            }
        );
        
        self.ir.forward_decls.push_str(&struct_code);

        Ok(())
    }
    
    pub fn codegen_impl_block(&mut self, impl_block: &ImplBlock) -> Result<()> {
        let loc = self.default_location();
        
        if !self.structs.contains_key(&impl_block.struct_name) {
            self.diagnostics.error(
                "UndefinedStruct",
                &format!("Struct '{}' is not defined", impl_block.struct_name),
                ErrorContext {
                    primary_location: loc.clone(),
                    secondary_locations: vec![],
                    help_message: Some(format!("Cannot implement methods for undefined struct '{}'.", impl_block.struct_name)),
                    suggestions: vec![
                        format!("Define struct '{}' before this impl block", impl_block.struct_name),
                        "Check for typos in the struct name".to_string(),
                    ],
                }
            );
        }
        
        if let Some(constructor_body) = &impl_block.constructor_body {
            self.codegen_constructor(&impl_block.struct_name, &impl_block.constructor_params, constructor_body);
        }
        
        for method in &impl_block.methods {
            self.codegen_impl_method(method, &impl_block.struct_name);
        }
        Ok(())
    }
    

    fn codegen_constructor(&mut self, struct_name: &str, params: &[(String, Type)], body: &[(String, Expr)]) -> Result<(), ()>  {
        let mut func_code = String::new();
        let func_name = format!("{}_new", struct_name);
        
        func_code.push_str(&format!("{} {}(", struct_name, func_name));
        
        let param_strs: Vec<String> = params.iter().map(|(name, ty)| format!("{} {}", ty.to_c_type(&self.arch), name)).collect();
        func_code.push_str(&param_strs.join(", "));
        func_code.push_str(") {\n");
        func_code.push_str(&format!("    {} instance;\n", struct_name));
        
        for (field_name, field_expr) in body {
            let mut temp_body = String::new();
            let (val_var, _) = self.codegen_expr(field_expr, &mut temp_body) .check_error();
            func_code.push_str(&temp_body);
            func_code.push_str(&format!("    instance.{} = {};\n", field_name, val_var));
        }
        
        func_code.push_str("    return instance;\n");
        func_code.push_str("}\n\n");
        
        self.ir.functions.push_str(&func_code);
        
        Ok(())  
    }

    pub fn codegen_impl_method(&mut self, method: &ImplMethod, struct_name: &str) {
        let loc = self.default_location();
        
        for (param_name, param_type, _) in &method.params {
            if matches!(param_type, Type::Void) {
                self.diagnostics.error(
                    "VoidParameter",
                    &format!("Parameter '{}' in method '{}' cannot be void", param_name, method.name),
                    ErrorContext {
                        primary_location: loc.clone(),
                        secondary_locations: vec![],
                        help_message: Some("Method parameters must have concrete types.".to_string()),
                        suggestions: vec![
                            format!("Change parameter '{}' to a concrete type", param_name),
                        ],
                    }
                );
            }
        }
        
        let mut func_code = String::new();
        let method_name = format!("{}_{}", struct_name, method.name);
        let return_c_type = method.return_type.to_c_type(&self.arch);

        func_code.push_str(&format!("{} {}(", return_c_type, method_name));
        
        if method.self_modifier.is_some() {
            func_code.push_str(&format!("{}* self", struct_name));
            if !method.params.is_empty() {
                func_code.push_str(", ");
            }
        }
        
        let param_strs: Vec<String> = method.params.iter().map(|(name, ty, _)| format!("{} {}", ty.to_c_type(&self.arch), name)).collect();
        func_code.push_str(&param_strs.join(", "));
        func_code.push_str(") {\n");
        
        let mut body_code = String::new();
        for stmt in &method.body {
            self.codegen_stmt(stmt, &mut body_code);
        }
        func_code.push_str(&body_code);
        
        if !matches!(method.return_type, Type::Void) {
            if !body_code.contains("return") {
                func_code.push_str("    return 0;\n");
            }
        } else {
            if !body_code.contains("return") {
                func_code.push_str("    return;\n");
            }
        }
        
        func_code.push_str("}\n\n");
        
        self.impl_methods.insert(
            (struct_name.to_string(), method.name.clone()),
            (method.params.iter().map(|(n, t, _)| (n.clone(), t.clone())).collect(), method.return_type.clone())
        );
        
        self.ir.functions.push_str(&func_code);
    }
    
    pub fn codegen_function(&mut self, func: &Function) {
        let loc = self.default_location();
        
        if self.user_functions.contains_key(&func.name) {
            self.diagnostics.error(
                "DuplicateFunction",
                &format!("Function '{}' is already defined", func.name),
                ErrorContext {
                    primary_location: loc.clone(),
                    secondary_locations: vec![],
                    help_message: Some("Each function must have a unique name.".to_string()),
                    suggestions: vec![
                        format!("Rename this function to something else"),
                        "Check for duplicate function definitions".to_string(),
                    ],
                }
            );
        }
        
        for (param_name, param_type, _) in &func.params {
            if matches!(param_type, Type::Void) {
                self.diagnostics.error(
                    "VoidParameter",
                    &format!("Parameter '{}' in function '{}' cannot be void", param_name, func.name),
                    ErrorContext {
                        primary_location: loc.clone(),
                        secondary_locations: vec![],
                        help_message: Some("Function parameters must have concrete types.".to_string()),
                        suggestions: vec![
                            format!("Change parameter '{}' to a concrete type", param_name),
                            "Remove this parameter if it's not needed".to_string(),
                        ],
                    }
                );
            }
        }
        
        let mut func_code = String::new();
        let return_c_type = func.return_type.to_c_type(&self.arch);

        func_code.push_str(&format!("{} {}(", return_c_type, func.name));
        
        let param_strs: Vec<String> = func.params.iter().map(|(name, ty, _)| {
                let c_type = ty.to_c_type(&self.arch);
                let c_name = format!("var_{}", name);
    
                self.vars.insert(name.clone(), (c_name.clone(), ty.clone()));
                format!("{} {}", c_type, c_name)
            }).collect();
        func_code.push_str(&param_strs.join(", "));
        func_code.push_str(") {\n");
        
        let mut body_code = String::new();
        for stmt in &func.body {
            self.codegen_stmt(stmt, &mut body_code);
        }
        func_code.push_str(&body_code);
        
        if matches!(func.return_type, Type::Void) {
            if !body_code.contains("return") {
                func_code.push_str("    return;\n");
            }
        }
        
        func_code.push_str("}\n\n");
        
        self.user_functions.insert(
            func.name.clone(),
            (func.params.iter().map(|(n, t, _)| (n.clone(), t.clone())).collect(), func.return_type.clone())
        );
        
        self.ir.functions.push_str(&func_code);
        
    }


    pub fn codegen_if(
        &mut self,
        cond: &Expr,
        then_body: &[Stmt],
        else_body: &Option<Vec<Stmt>>,
        body: &mut String,
    ) -> Result<(), ()> {
        let (cond_var, _cond_ty) = self.codegen_expr(cond, body) .check_error();

        body.push_str(&format!("if ({}) {{\n", cond_var));

        for stmt in then_body {
            self.codegen_stmt(stmt, body);
        }

        body.push_str("}\n");

        if let Some(else_stmts) = else_body {
            body.push_str("else {\n");
            for stmt in else_stmts {
                self.codegen_stmt(stmt, body);
            }
            body.push_str("}\n");
        }

        Ok(())
    }
    pub fn codegen_enum_definition(&mut self, enum_def: &EnumDef) -> Result<(), ()> {
        let mut variants = Vec::new();
        for variant in &enum_def.variants {
            match variant {
                EnumVariant::Simple(name) => variants.push((name.clone(), None)),
                EnumVariant::Tuple(name, types) => {
                    variants.push((name.clone(), Some(Type::Tuple { fields: types.clone() })));
                }
                EnumVariant::Struct(name, fields) => {
                     let payload_name = format!("{}_{}_Payload", enum_def.name, name);
                     let struct_def = StructDef {
                        name: payload_name.clone(),
                        fields: fields.clone(),
                     };
                     
                     
                    let reg_fields = fields.iter().map(|f| (f.name.clone(), f.ty.clone())).collect();
                    self.type_registry.register_struct(payload_name.clone(), reg_fields);
                    self.codegen_struct_definition(&struct_def);

                     variants.push((name.clone(), Some(Type::Struct { name: payload_name })));
                }
            }
        }
        self.type_registry.register_enum(enum_def.name.clone(), variants, enum_def.is_public);
        
        if let Some(def_code) = self.type_registry.generate_enum_definition(&enum_def.name, &self.config.arch) {
            self.ir.forward_decls.push_str(&def_code);
        }

        Ok(())
    }

    
    pub fn codegen_scope(&mut self, stmts: &[Stmt], body: &mut String) -> Result<(), ()>{
        self.scope_depth += 1;
        let prev_owned_vars = self.owned_vars.clone();
        
        body.push_str("{\n");
        for stmt in stmts {
            self.codegen_stmt(stmt, body);
        }
        
        let current_owned = self.owned_vars.clone();
        for var_name in current_owned {
            if !prev_owned_vars.contains(&var_name) {
                if let Some((c_name, _)) = self.vars.get(&var_name) {
                    body.push_str(&format!("free({});\n", c_name));
                }
                self.owned_vars.remove(&var_name);
            }
        }
        
        body.push_str("}\n");
        self.scope_depth -= 1;

        Ok(())
    }

    pub fn codegen_while(&mut self, cond: &Expr, loop_body: &[Stmt], body: &mut String, _loc: SourceLocation)  -> Result<(), ()> {
        let loop_label = self.fresh_label();
        let end_label = self.fresh_label();
        let (cond_var, _) = self.codegen_expr(cond, body).check_error();
        
        body.push_str(&format!("{}:\n", loop_label));
        body.push_str(&format!("if (!{}) goto {};\n", cond_var, end_label));
        
        for stmt in loop_body {
            self.codegen_stmt(stmt, body);
        }
        
        body.push_str(&format!("goto {};\n", loop_label));
        body.push_str(&format!("{}:\n", end_label));

        Ok(()) 
    }

    pub fn codegen_for(&mut self, var: &str, iter: &Expr, loop_body: &[Stmt], body: &mut String, _loc: SourceLocation) -> Result<(), ()> {
        let (iter_var, iter_ty) = self.codegen_expr(iter, body) .check_error();
        let loop_label = self.fresh_label();
        let end_label = self.fresh_label();
        let idx_var = self.fresh_var();
        let (size_expr, elem_access, elem_type) = match &iter_ty {
            Type::Array { element, size: Some(size) } => {
                (format!("{}", size), format!("{}[{}]", iter_var, idx_var), *element.clone())
            }
            Type::Array { element, size: None } => {
                (format!("{}.len", iter_var), format!("{}.ptr[{}]", iter_var, idx_var), *element.clone())
            }
        
            Type::MultiArray { element, dimensions } => {
                if dimensions.is_empty() {
                    ("0".to_string(), "NULL".to_string(), *element.clone())
                } else {
                    (format!("{}", dimensions[0]), format!("{}[{}]", iter_var, idx_var), *element.clone())
                }
            }
            _ => {
                ("1".to_string(), format!("{}[{}]", iter_var, idx_var), Type::i32())
            }
        };
        
        let c_name = format!("var_{}", var);
        let c_type = elem_type.to_c_type(&self.arch);
        
        body.push_str(&format!("size_t {} = 0;\n", idx_var));
        body.push_str(&format!("{}:\n", loop_label));
        body.push_str(&format!("if ({} >= {}) goto {};\n", idx_var, size_expr, end_label));
        body.push_str(&format!("{} {} = {};\n", c_type, c_name, elem_access));
        
        self.vars.insert(var.to_string(), (c_name, elem_type));
        
        for stmt in loop_body {
            self.codegen_stmt(stmt, body);
        }
        
        body.push_str(&format!("{}++;\n", idx_var));
        body.push_str(&format!("goto {};\n", loop_label));
        body.push_str(&format!("{}:\n", end_label));
        
        Ok(())
    }

}