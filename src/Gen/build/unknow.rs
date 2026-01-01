use crate::import::*;

impl Codegen {
    
    pub fn codegen_var(&mut self, name: &str, loc: SourceLocation) -> Result<(String, Type), ()> {
        if let Some((c_name, ty)) = self.vars.get(name) {
            return Ok((c_name.clone(), ty.clone()));
        }
        
        self.diagnostics.error(
            "UndefinedVariable",
            &format!("Variable '{}' is not defined", name),
            ErrorContext {
                primary_location: loc,
                secondary_locations: vec![],
                help_message: Some(format!("Variable '{}' has not been declared in this scope.", name)),
                suggestions: vec![
                    format!("Declare '{}' before using it", name),
                    "Check for typos in the variable name".to_string(),
                ],
            }
        );
        Err(())
    }

    pub fn codegen_assign(&mut self, name: &str, value: &Expr, body: &mut String, loc: SourceLocation) -> Result<(), ()> {
        let (val_var, val_ty) = self.codegen_expr(value, body) .check_error();

        if let Some((c_name, var_ty)) = self.vars.get(name) {
            if !self.types_compatible(&var_ty, &val_ty) {
                self.diagnostics.error(
                    "TypeMismatch",
                    &format!("Cannot assign {} to variable of type {}", val_ty.name(), var_ty.name()),
                    type_mismatch_error(
                        &var_ty.name(),
                        &val_ty.name(),
                        loc.clone(),
                        value.location(),
                    )
                );
                return Err(());
            }
            
            let c_name = c_name.clone();
            body.push_str(&format!("{} = {};\n", c_name, val_var));
            Ok(())
        } else {
            self.diagnostics.error(
                "UndefinedVariable",
                &format!("Variable '{}' is not defined", name),
                ErrorContext {
                    primary_location: loc,
                    secondary_locations: vec![],
                    help_message: Some(format!("Cannot assign to undefined variable '{}'.", name)),
                    suggestions: vec![
                        format!("Declare '{}' before assignment", name),
                        format!("Use 'let {} = ...' to declare and initialize", name),
                    ],
                }
            );
            Err(())
        }
    }

    pub fn codegen_compound_assign(&mut self, name: &str, op: &str, value: &Expr, body: &mut String, loc: SourceLocation) -> Result<(), ()> {
        let (val_var, val_ty) = self.codegen_expr(value, body) .check_error();
        
        if let Some((c_name, var_ty)) = self.vars.get(name) {
            if matches!(var_ty, Type::Void) || matches!(val_ty, Type::Void) {
                self.diagnostics.error(
                    "VoidOperation",
                    "Cannot perform compound assignment on void type",
                    void_operation_error(op, loc)
                );
                return Err(());
            }
            
            let c_name = c_name.clone();
            body.push_str(&format!("{} {} {};\n", c_name, op, val_var));
            Ok(())
        } else {
            self.diagnostics.error(
                "UndefinedVariable",
                &format!("Variable '{}' is not defined", name),
                ErrorContext {
                    primary_location: loc,
                    secondary_locations: vec![],
                    help_message: Some(format!("Cannot perform compound assignment on undefined variable '{}'.", name)),
                    suggestions: vec![format!("Declare '{}' before using compound assignment", name)],
                }
            );
            Err(())
        }
    }
    pub fn codegen_call_expr(&mut self, func: &str, args: &[Expr], body: &mut String, loc: SourceLocation) -> Result<(String, Type), ()> {
        match func {
            "print" => {
                let mut format_str = String::new();
                let mut arg_list = Vec::new();
                
                for arg in args {
                    let (var, ty) = self.codegen_expr(arg, body) .check_error();
                    match ty {
                        Type::Int { .. } => {
                            format_str.push_str("%d ");
                            arg_list.push(var);
                        }
                        Type::Float { .. } => {
                            format_str.push_str("%f ");
                            arg_list.push(var);
                        }
                        Type::Str { .. } => {
                            format_str.push_str("%s ");
                            arg_list.push(var);
                        }
                        _ => {
                            format_str.push_str("%p ");
                            arg_list.push(var);
                        }
                    }
                }
                format_str.push_str("\\n");
                
                let tmp = self.fresh_var();
                let args_final = if arg_list.is_empty() { String::new() } else { format!(", {}", arg_list.join(", ")) };
                body.push_str(&format!("int32_t {} = printf(\"{}\"{});\n", tmp, format_str, args_final));
                return Ok((tmp, Type::i32()));
            }
            
            "chars" => {
                if args.len() != 1 { return Err(()); }
                self.codegen_chars(&args[0], body)
            }
            "have" | "contain" | "contains" | "has" => {
                if args.len() != 2 { return Err(()); }
                self.codegen_have(&args[0], &args[1], body)
            }
            "is_not_empty" => {
                if args.len() != 1 { return Err(()); }
                self.codegen_is_not_empty(&args[0], body)
            }
            "collect" => {
                if args.len() != 1 { return Err(()); }
                self.codegen_collect(&args[0], body)
            }
            "contain_all" => {
                if args.len() != 2 { return Err(()); }
                if let Expr::Array(items) = &args[1] {
                    self.codegen_contain_all(&args[0], items, body)
                } else {
                    Err(())
                }
            }
            "index" => {
                if args.len() < 2 { return Err(()); }
                let (arr, indices) = args.split_first().unwrap();
                self.codegen_index(arr, indices, body)
            }
            "index_of" => {
                if args.len() != 2 { return Err(()); }
                self.codegen_index_of(&args[0], &args[1], body)
            }

            "array" => {
                self.codegen_array(args, body)
            }
            "some" => {
                if args.len() != 1 { return Err(()); }
                self.codegen_some(&args[0], body)
            }
            "none" => {
                let expected = None;
                self.codegen_none(expected, body)
            }
            "ok" => {
                if args.len() != 1 { return Err(()); }
                self.codegen_result_ok(&args[0], body)
            }
            "err" => {
                if args.len() != 1 { return Err(()); }
                self.codegen_result_err(&args[0], body)
            }
            "unwrap" => {
                if args.len() != 1 { return Err(()); }
                self.codegen_unwrap(&args[0], body)
            }
            "unwrap_or" => {
                if args.len() != 2 { return Err(()); }
                self.codegen_unwrap_or(&args[0], &args[1], body)
            }
            "is_some" | "is_none" => {
                if args.len() != 1 { return Err(()); }
                self.codegen_option_method(&args[0], func, body)
            }
            "array_get" => {
                if args.len() != 2 { return Err(()); }
                self.codegen_array_get(&args[0], &args[1], body)
            }
            "wait" => {
                if args.len() != 1 { return Err(()); }
                self.codegen_wait(&args[0], body)
            }
            "tuple" => {
                self.codegen_tuple(args, body)
            }
            "is_empty" => {
                if args.len() != 1 { return Err(()); }
                self.codegen_is_empty(&args[0], body)
            }
            "filter" => {
                if args.len() != 2 { return Err(()); }
                self.codegen_filter(&args[0], &args[1], body)
            }
            "panic" => {
                if args.len() != 1 { return Err(()); }
                self.codegen_panic(&args[0], body)
            }
            _ => {
                self.codegen_std_call(func, args, body, loc)
            }
        }
    }

    pub fn codegen_member_access(&mut self, obj: &Expr, field: &str, body: &mut String, loc: SourceLocation) -> Result<(String, Type), ()> {
        let (obj_var, obj_ty) = self.codegen_expr(obj, body) .check_error();
        
        if matches!(obj_ty, Type::Void) {
            self.diagnostics.error(
                "VoidMemberAccess",
                "Cannot access member of void type",
                void_operation_error("member access", loc)
            );
            return Err(());
        }
        
        let tmp = self.fresh_var();
        body.push_str(&format!("int32_t {} = {}.{};\n", tmp, obj_var, field));
        Ok((tmp, Type::i32()))
    }

    pub fn codegen_cast_target(&mut self, expr: &Expr, target: &CastTarget, body: &mut String, loc: SourceLocation) -> Result<(String, Type), ()> {
        if let CastTarget::Type(ty) = target {
            self.codegen_cast(expr, ty, body, loc)
        } else {
            Err(())
        }
    }

    pub fn codegen_index_assign(&mut self, arr: &Expr, indices: &[Expr], value: &Expr, body: &mut String, _loc: SourceLocation) -> Result<(), ()> {
        let (arr_var, _arr_ty) = self.codegen_expr(arr, body).check_error();
        let (val_var, _val_ty) = self.codegen_expr(value, body).check_error();
        
        let mut index_str = arr_var.clone();
        
        for idx in indices {
            let (idx_var, _idx_ty) = self.codegen_expr(idx, body).check_error();
            index_str = format!("{}[{}]", index_str, idx_var);
        }
        
        body.push_str(&format!("{} = {};\n", index_str, val_var));

        Ok(())
    }

    pub fn codegen_member_assign(&mut self, obj: &Expr, field: &str, value: &Expr, body: &mut String, loc: SourceLocation) -> Result<(), ()> {
        let (obj_var, obj_ty) = self.codegen_expr(obj, body).check_error();
        let (val_var, _val_ty) = self.codegen_expr(value, body).check_error();

        if matches!(obj_ty, Type::Void) {
            self.diagnostics.error(
                "VoidMemberAssign",
                "Cannot assign to member of void type",
                void_operation_error("member assignment", loc)
            );
        }

        body.push_str(&format!("{}.{} = {};\n", obj_var, field, val_var));

        Ok(())
    }

    pub fn codegen_call_stmt(&mut self, func: &str, args: &[Expr], body: &mut String, loc: SourceLocation) -> Result<(), ()> {
        if func == "print" {
            self.codegen_call_expr(func, args, body, loc).check_error();
        }

        let mut arg_vars = Vec::new();
        
        for arg in args {
            let (var, _ty) = self.codegen_expr(arg, body).check_error();
            arg_vars.push(var);
        }

        let args_str = arg_vars.join(", ");
        body.push_str(&format!("{}({});\n", func, args_str));

        Ok(())
    }

    pub fn codegen_program(&mut self, functions: &[Function]) -> Result<(), ()> {
        for func in functions {
            self.codegen_function(func)
        }
 
        Ok(())
    }

    pub fn finalize(self) -> Result<String, ()> {
        if self.diagnostics.has_errors() {
            self.diagnostics.print_summary();
            Err(())
        } else {
            Ok(self.ir.finalize())
        }
    }

    pub fn codegen_static_method(&mut self, type_name: &str, method: &str, args: &[Expr], body: &mut String, _loc: SourceLocation) -> Result<(String, Type), ()> {
        let mut arg_vars = Vec::new();
        let method_name = format!("{}_{}", type_name, method);
        let tmp = self.fresh_var();
        let args_str = arg_vars.join(", ");
        
        for arg in args {
            let (var, _ty) = self.codegen_expr(arg, body) .check_error();
            arg_vars.push(var);
        }
        
        
        body.push_str(&format!("{} {} = {}({});\n", type_name, tmp, method_name, args_str));
        Ok((tmp, Type::Struct { name: type_name.to_string() }))
    }

    pub fn codegen_cast(&mut self, expr: &Expr, target_ty: &Type, body: &mut String, loc: SourceLocation) -> Result<(String, Type), ()> {
        let (var, source_ty) = self.codegen_expr(expr, body) .check_error();

        if matches!(source_ty, Type::Ptr(_)) && !matches!(target_ty, Type::Ptr(_) | Type::RawPtr(_)) {
            self.diagnostics.warning(
                "UnsafeCast",
                "Casting pointer to non-pointer type may be unsafe",
                ErrorContext {
                    primary_location: loc.clone(),
                    secondary_locations: vec![],
                    help_message: Some("This cast may lose pointer information.".to_string()),
                    suggestions: vec!["Ensure this cast is intentional".to_string()],
                }
            );
        }
        
        let c_type = target_ty.to_c_type(&self.arch);
        let tmp = self.fresh_var();

        body.push_str(&format!("{} {} = ({}){};\n", c_type, tmp, c_type, var));
        Ok((tmp, target_ty.clone()))
    }
    pub fn types_compatible(&self, ty1: &Type, ty2: &Type) -> bool {
        match (ty1, ty2) {
            (Type::Int { bits: b1, signed: s1 }, Type::Int { bits: b2, signed: s2 }) => b1 == b2 && s1 == s2,
            (Type::Float { bits: b1 }, Type::Float { bits: b2 }) => b1 == b2,
            (Type::Bool, Type::Bool) => true,
            (Type::Void, Type::Void) => true,
            (Type::Char { .. }, Type::Char { .. }) => true,
            (Type::Str { .. }, Type::Str { .. }) => true,
            (Type::Ptr(inner1), Type::Ptr(inner2)) => self.types_compatible(inner1, inner2),
            (Type::Struct { name: n1 }, Type::Struct { name: n2 }) => n1 == n2,
            (Type::Array { element: e1, size: s1 }, Type::Array { element: e2, size: s2 }) => {
                self.types_compatible(e1, e2) && (s1 == s2)
            },
            (Type::Tuple { fields: f1 }, Type::Tuple { fields: f2 }) => {
                if f1.len() != f2.len() { return false; }
                f1.iter().zip(f2.iter()).all(|(t1, t2)| self.types_compatible(t1, t2))
            },
            (Type::Option { inner: i1 }, Type::Option { inner: i2 }) => self.types_compatible(i1, i2),
            (Type::Result { ok: o1, err: e1 }, Type::Result { ok: o2, err: e2 }) => {
                self.types_compatible(o1, o2) && self.types_compatible(e1, e2)
            },
            _ => false,
        }
    }
}

