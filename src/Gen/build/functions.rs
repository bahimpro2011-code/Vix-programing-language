use crate::import::*;

impl Codegen {
 
    
    pub fn codegen_chars(&mut self, expr: &Expr, body: &mut String) -> Result<(String, Type), ()> {
        let (str_var, _) = self.codegen_expr(expr, body) .check_error();
        let tmp = self.fresh_var();
        let _func_name = format!("{}_chars", tmp); 
        let func_def = format!(
            "char* {}_chars(const char* str) {{
    size_t len = strlen(str);
    char* result = malloc((len + 1) * sizeof(char));
    if (result != NULL) {{
        strcpy(result, str);
    }}
    return result;
}}\n", tmp
        );
        
        self.ir.add_function(func_def);
        body.push_str(&format!("char* {} = {}_chars({});\n", tmp, tmp, str_var));
        
        Ok((tmp, Type::Array { element: Box::new(Type::Char { bits: 8, signed: true }), size: None }))
    }

    pub fn codegen_have(&mut self, obj: &Expr, item: &Expr, body: &mut String) -> Result<(String, Type), ()> {
        let (obj_var, obj_ty) = self.codegen_expr(obj, body) .check_error();
        let (item_var, _) = self.codegen_expr(item, body) .check_error();
        let tmp = self.fresh_var();
        
        match &obj_ty {
            Type::Array { element: _, size: Some(size) } => {
                body.push_str(&format!("bool {} = false;\n", tmp));
                body.push_str(&format!("for (int i = 0; i < {}; i++) {{\n", size));
                body.push_str(&format!("    if ({}[i] == {}) {{\n", obj_var, item_var));
                body.push_str(&format!("        {} = true;\n", tmp));
                body.push_str("        break;\n");
                body.push_str("    }\n");
                body.push_str("}\n");
            }
            Type::Array { element: _, size: None } => {
                body.push_str(&format!("bool {} = false;\n", tmp));
                body.push_str(&format!("for (int i = 0; i < {}.len; i++) {{\n", obj_var));
                body.push_str(&format!("    if ({}.ptr[i] == {}) {{\n", obj_var, item_var));
                body.push_str(&format!("        {} = true;\n", tmp));
                body.push_str("        break;\n");
                body.push_str("    }\n");
                body.push_str("}\n");
            }
            Type::Ptr(_) => {
                body.push_str(&format!("bool {} = ({} == {});\n", tmp, obj_var, item_var));
            }
            _ => {
                body.push_str(&format!("bool {} = ({} == {});\n", tmp, obj_var, item_var));
            }
        }
        
        Ok((tmp, Type::Bool))
    }

    pub fn codegen_is_not_empty(&mut self, expr: &Expr, body: &mut String) -> Result<(String, Type), ()> {
        let (var, ty) = self.codegen_expr(expr, body) .check_error();
        let tmp = self.fresh_var();
        
        match &ty {
            Type::Array { size: Some(size), .. } => {
                body.push_str(&format!("bool {} = ({} > 0);\n", tmp, size));
            }
            Type::Array { size: None, .. } => {
                body.push_str(&format!("bool {} = ({}.len > 0);\n", tmp, var));
            }
            Type::Ptr(_) => {
                body.push_str(&format!("bool {} = ({} != NULL);\n", tmp, var));
            }
            Type::Str {..} => {
                body.push_str(&format!("bool {} = (strlen({}) > 0);\n", tmp, var));
            }
            _ => {
                body.push_str(&format!("bool {} = ({} != 0);\n", tmp, var));
            }
        }
        
        Ok((tmp, Type::Bool))
    }

    pub fn codegen_collect(&mut self, expr: &Expr, body: &mut String) -> Result<(String, Type), ()> {
        let (var, ty) = self.codegen_expr(expr, body) .check_error();
        let tmp = self.fresh_var();

        match &ty {
            Type::Array { .. } => {
                body.push_str(&format!("void* {} = {};\n", tmp, var));
            }
            _ => {
                body.push_str(&format!("void* {} = {};\n", tmp, var));
            }
        }
        Ok((tmp, Type::Ptr(Box::new(Type::Void))))
    }

    pub fn codegen_contain_all(&mut self, obj: &Expr, items: &[Expr], body: &mut String) -> Result<(String, Type), ()> {
        let (obj_var, obj_ty) = self.codegen_expr(obj, body) .check_error();
        let tmp = self.fresh_var();
    
        body.push_str(&format!("bool {} = true;\n", tmp));
        
        for (i, item) in items.iter().enumerate() {
            let (item_var, _) = self.codegen_expr(item, body) .check_error();
            let temp_check = format!("temp_check_{}", i);
            
            match &obj_ty {
                Type::Array { element: _, size: Some(size) } => {
                    body.push_str(&format!("bool {} = false;\n", temp_check));
                    body.push_str(&format!("for (int i = 0; i < {}; i++) {{\n", size));
                    body.push_str(&format!("    if ({}[i] == {}) {{\n", obj_var, item_var));
                    body.push_str(&format!("        {} = true;\n", temp_check));
                    body.push_str("        break;\n");
                    body.push_str("    }\n");
                    body.push_str("}\n");
                }
                Type::Array { element: _, size: None } => {
                    body.push_str(&format!("bool {} = false;\n", temp_check));
                    body.push_str(&format!("for (int i = 0; i < {}.len; i++) {{\n", obj_var));
                    body.push_str(&format!("    if ({}.ptr[i] == {}) {{\n", obj_var, item_var));
                    body.push_str(&format!("        {} = true;\n", temp_check));
                    body.push_str("        break;\n");
                    body.push_str("    }\n");
                    body.push_str("}\n");
                }
                _ => {
                    body.push_str(&format!("bool {} = ({} == {});\n", temp_check, obj_var, item_var));
                }
            }
            
            body.push_str(&format!("{} = {} && {};\n", tmp, tmp, temp_check));
        }
        
        Ok((tmp, Type::Bool))
    }

    pub fn codegen_contain(&mut self, obj: &Expr, item: &Expr, body: &mut String) -> Result<(String, Type), ()> {
        self.codegen_have(obj, item, body)
    }

    pub fn codegen_index(&mut self, arr: &Expr, indices: &[Expr], body: &mut String) -> Result<(String, Type), ()> {
        let (arr_var, arr_ty) = self.codegen_expr(arr, body) .check_error();
        
        let elem_ty = match arr_ty {
            Type::Array { element, .. } => *element,
            Type::Ptr(inner) => *inner,
            _ => {
                return Err(());
            }
        };
        
        if matches!(elem_ty, Type::Void) {
            return Err(());
        }
        
        let c_type = elem_ty.to_c_type(&self.arch);
        
        let mut index_str = arr_var.clone();
        for idx in indices {
            let (idx_var, _) = self.codegen_expr(idx, body) .check_error();
            index_str = format!("{}[{}]", index_str, idx_var);
        }
        
        let tmp = self.fresh_var();
        body.push_str(&format!("{} {} = {};\n", c_type, tmp, index_str));
        Ok((tmp, elem_ty))
    }

    pub fn codegen_index_of(&mut self, obj: &Expr, item: &Expr, body: &mut String) -> Result<(String, Type), ()> {
        let (obj_var, obj_ty) = self.codegen_expr(obj, body) .check_error();
        let (item_var, _) = self.codegen_expr(item, body) .check_error();
        let tmp = self.fresh_var();
        
        match &obj_ty {
            Type::Array { element: _, size: Some(size) } => {
                body.push_str(&format!("int {} = -1;\n", tmp));
                body.push_str(&format!("for (int i = 0; i < {}; i++) {{\n", size));
                body.push_str(&format!("    if ({}[i] == {}) {{\n", obj_var, item_var));
                body.push_str(&format!("        {} = i;\n", tmp));
                body.push_str("        break;\n");
                body.push_str("    }\n");
                body.push_str("}\n");
            }
            Type::Array { element: _, size: None } => {
                body.push_str(&format!("int {} = -1;\n", tmp));
                body.push_str(&format!("for (int i = 0; i < {}.len; i++) {{\n", obj_var));
                body.push_str(&format!("    if ({}.ptr[i] == {}) {{\n", obj_var, item_var));
                body.push_str(&format!("        {} = i;\n", tmp));
                body.push_str("        break;\n");
                body.push_str("    }\n");
                body.push_str("}\n");
            }
            _ => {
                body.push_str(&format!("int {} = ({} == {}) ? 0 : -1;\n", tmp, obj_var, item_var));
            }
        }
        
        Ok((tmp, Type::Int { bits: 32, signed: true }))
    }

    pub fn codegen_reference_to(&mut self, ty: &Type, body: &mut String) -> Result<(String, Type), ()> {
        let tmp: String = self.fresh_var();
        let type_name = ty.name();
            
        let type_id = type_name.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
            
        body.push_str(&format!("uint32_t {} = {};\n", tmp, type_id));
        Ok((tmp, Type::u32()))
    }

    pub fn codegen_array(&mut self, elements: &[Expr], body: &mut String) -> Result<(String, Type), ()> {
        if elements.is_empty() {
            return Err(());
        }
        
        let mut elem_vars = Vec::new();
        let mut elem_type = None;
        
        for elem in elements {
            let (var, ty) = self.codegen_expr(elem, body) .check_error();
            if matches!(ty, Type::Void) {
                return Err(());
            }
            if elem_type.is_none() {
                elem_type = Some(ty);
            }
            elem_vars.push(var);
        }
        
        let elem_type = elem_type.unwrap();
        let c_type = elem_type.to_c_type(&self.arch);
        
        let tmp = self.fresh_var();
        let elems_str = elem_vars.join(", ");
        
        body.push_str(&format!("{} {}[] = {{{}}};\n", c_type, tmp, elems_str));
        
        Ok((tmp, Type::Array { 
            element: Box::new(elem_type), 
            size: Some(elements.len()) 
        }))
    }

    pub fn codegen_some(&mut self, inner: &Expr, body: &mut String) -> Result<(String, Type), ()> {
        let (val_var, val_ty) = self.codegen_expr(inner, body) .check_error();
        let tmp = self.fresh_var();
        
 
        let opt_type = Type::Option { inner: Box::new(val_ty.clone()) };
        if let Some(def) = self.type_registry.generate_type_definition(&opt_type, &self.arch) {
            self.ir.add_function(def); 
        }
        
        let c_type = opt_type.to_c_type(&self.arch);
        body.push_str(&format!("{} {} = {{ .tag = 1, .value = {} }};\n", c_type, tmp, val_var));
        
        Ok((tmp, opt_type))
    }
    
    pub fn codegen_none(&mut self, expected_type: Option<&Type>, body: &mut String) -> Result<(String, Type), ()> {
        let tmp = self.fresh_var();
        
 
        let inner_type = expected_type
            .and_then(|t| match t {
                Type::Option { inner } => Some(inner.as_ref().clone()),
                _ => None,
            })
            .unwrap_or(Type::Void);
        
        let opt_type = Type::Option { inner: Box::new(inner_type.clone()) };
        if let Some(def) = self.type_registry.generate_type_definition(&opt_type, &self.arch) {
            self.ir.add_function(def); 
        }
        
        let c_type = opt_type.to_c_type(&self.arch);
        body.push_str(&format!("{} {} = {{ .tag = 0 }};\n", c_type, tmp));
        
        Ok((tmp, opt_type))
    }
    
    pub fn codegen_result_ok(&mut self, value: &Expr, body: &mut String) -> Result<(String, Type), ()> {
        let (val_var, val_ty) = self.codegen_expr(value, body) .check_error();
        let tmp = self.fresh_var();
        
        let result_type = Type::Result { 
            ok: Box::new(val_ty.clone()), 
            err: Box::new(Type::Void) 
        };
        
        if let Some(def) = self.type_registry.generate_type_definition(&result_type, &self.arch) {
            self.ir.add_function(def); 
        }
        
        let c_type = result_type.to_c_type(&self.arch);
        body.push_str(&format!("{} {} = {{ .tag = 0, .data.ok = {} }};\n", c_type, tmp, val_var));
        
        Ok((tmp, result_type))
    }
    
    pub fn codegen_result_err(&mut self, value: &Expr, body: &mut String) -> Result<(String, Type), ()> {
        let (val_var, val_ty) = self.codegen_expr(value, body) .check_error();
        let tmp = self.fresh_var();
        
        let result_type = Type::Result { 
            ok: Box::new(Type::Void), 
            err: Box::new(val_ty.clone()) 
        };
        
        if let Some(def) = self.type_registry.generate_type_definition(&result_type, &self.arch) {
            self.ir.add_function(def); 
        }
        
        let c_type = result_type.to_c_type(&self.arch);
        body.push_str(&format!("{} {} = {{ .tag = 1, .data.err = {} }};\n", c_type, tmp, val_var));
        
        Ok((tmp, result_type))
    }
    
    pub fn codegen_unwrap(&mut self, expr: &Expr, body: &mut String) -> Result<(String, Type), ()> {
        let (var, ty) = self.codegen_expr(expr, body) .check_error();
        let tmp = self.fresh_var();
        
        match &ty {
            Type::Option { inner } => {
                let c_type = inner.to_c_type(&self.arch);
 
                body.push_str(&format!("if ({}.tag == 0) {{\n", var));
                body.push_str("    fprintf(stderr, \"unwrap called on None\\n\");\n");
                body.push_str("    exit(1);\n");
                body.push_str("}\n");
                body.push_str(&format!("{} {} = {}.value;\n", c_type, tmp, var));
                Ok((tmp, *inner.clone()))
            }
            Type::Result { ok, .. } => {
                let c_type = ok.to_c_type(&self.arch);
 
                body.push_str(&format!("if ({}.tag != 0) {{\n", var));
                body.push_str("    fprintf(stderr, \"unwrap called on Err\\n\");\n");
                body.push_str("    exit(1);\n");
                body.push_str("}\n");
                body.push_str(&format!("{} {} = {}.data.ok;\n", c_type, tmp, var));
                Ok((tmp, *ok.clone()))
            }
            Type::Ptr(inner) => {
                let c_type = inner.to_c_type(&self.arch);
                body.push_str(&format!("if ({} == NULL) {{\n", var));
                body.push_str("    fprintf(stderr, \"unwrap called on null pointer\\n\");\n");
                body.push_str("    exit(1);\n");
                body.push_str("}\n");
                body.push_str(&format!("{} {} = *{};\n", c_type, tmp, var));
                Ok((tmp, *inner.clone()))
            }
            _ => {
 
                Ok((var, ty))
            }
        }
    }
    
    pub fn codegen_unwrap_or(&mut self, expr: &Expr, default: &Expr, body: &mut String) -> Result<(String, Type), ()> {
        let (var, ty) = self.codegen_expr(expr, body) .check_error();
        let (default_var, default_ty) = self.codegen_expr(default, body) .check_error();
        let tmp = self.fresh_var();
        
        match &ty {
            Type::Option { inner } => {
                let c_type = inner.to_c_type(&self.arch);
                body.push_str(&format!("{} {} = ({}.tag == 1) ? {}.value : {};\n", 
                    c_type, tmp, var, var, default_var));
                Ok((tmp, *inner.clone()))
            }
            Type::Result { ok, .. } => {
                let c_type = ok.to_c_type(&self.arch);
                body.push_str(&format!("{} {} = ({}.tag == 0) ? {}.data.ok : {};\n", 
                    c_type, tmp, var, var, default_var));
                Ok((tmp, *ok.clone()))
            }
            Type::Ptr(_) => {
                body.push_str(&format!("auto {} = ({} != NULL) ? *{} : {};\n", 
                    tmp, var, var, default_var));
                Ok((tmp, default_ty))
            }
            _ => {
 
                Ok((var, ty))
            }
        }
    }
    
    pub fn codegen_option_method(&mut self, obj: &Expr, method: &str, body: &mut String) -> Result<(String, Type), ()> {
        let (obj_var, obj_ty) = self.codegen_expr(obj, body) .check_error();
        let tmp = self.fresh_var();
        
        match method {
            "is_some" => {
                match &obj_ty {
                    Type::Option { .. } => {
                        body.push_str(&format!("bool {} = ({}.tag == 1);\n", tmp, obj_var));
                    }
                    Type::Ptr(_) => {
                        body.push_str(&format!("bool {} = ({} != NULL);\n", tmp, obj_var));
                    }
                    _ => {
                        body.push_str(&format!("bool {} = true;\n", tmp));
                    }
                }
                Ok((tmp, Type::Bool))
            }
            "is_none" => {
                match &obj_ty {
                    Type::Option { .. } => {
                        body.push_str(&format!("bool {} = ({}.tag == 0);\n", tmp, obj_var));
                    }
                    Type::Ptr(_) => {
                        body.push_str(&format!("bool {} = ({} == NULL);\n", tmp, obj_var));
                    }
                    _ => {
                        body.push_str(&format!("bool {} = false;\n", tmp));
                    }
                }
                Ok((tmp, Type::Bool))
            }
            _ => {
 
                body.push_str(&format!("int32_t {} = 0;\n", tmp));
                Ok((tmp, Type::i32()))
            }
        }
    }

    pub fn codegen_array_get(&mut self, obj: &Expr, reference: &Expr, body: &mut String) -> Result<(String, Type), ()> {
        let (obj_var, _obj_ty) = self.codegen_expr(obj, body).check_error();
        let (ref_var, ref_ty) = self.codegen_expr(reference, body).check_error();
        let tmp = self.fresh_var();
        let _ref_var = ref_var; 

        body.push_str(&format!("void* {} = array_get_by_type({}, {});\n", tmp, obj_var, _ref_var));
        Ok((tmp, Type::Option { inner: Box::new(ref_ty) }))
    }

    pub fn codegen_wait(&mut self, expr: &Expr, body: &mut String) -> Result<(String, Type), ()> {
        let (var, ty) = self.codegen_expr(expr, body) .check_error();
        let tmp = self.fresh_var();
        body.push_str(&format!("{}* {} = &{};\n", ty.to_c_type(&self.arch), tmp, var));
        Ok((tmp, Type::Ptr(Box::new(ty))))
    }

    pub fn codegen_tuple(&mut self, elements: &[Expr], body: &mut String) -> Result<(String, Type), ()> {
        if elements.is_empty() {
            let tmp = self.fresh_var();
            body.push_str(&format!("struct {{}} {} = {{}};\n", tmp));
            return Ok((tmp, Type::Tuple { fields: vec![] }));
        }
        
        let mut elem_vars = Vec::new();
        let mut elem_types = Vec::new();
        
        for elem in elements {
            let (var, ty) = self.codegen_expr(elem, body) .check_error();
            elem_vars.push(var);
            elem_types.push(ty);
        }
        
 
        let tmp = self.fresh_var();
        let fields_def: String = elem_types.iter().enumerate()
            .map(|(i, ty)| format!("{} _{}", ty.to_c_type(&self.arch), i))
            .collect::<Vec<_>>()
            .join("; ");
        let fields_init: String = elem_vars.iter().enumerate()
            .map(|(i, var)| format!("._{} = {}", i, var))
            .collect::<Vec<_>>()
            .join(", ");
        
        body.push_str(&format!("struct {{ {}; }} {} = {{ {} }};\n", fields_def, tmp, fields_init));
        
        Ok((tmp, Type::Tuple { fields: elem_types }))
    }

    pub fn codegen_method_call(&mut self, obj: &Expr, method: &str, args: &[Expr], body: &mut String, _loc: SourceLocation) -> Result<(String, Type), ()> {
        let (obj_var, obj_ty) = self.codegen_expr(obj, body) .check_error();
        
 
        match (method, &obj_ty) {
 
            ("len", Type::Array { size: Some(size), .. }) => {
                let tmp = self.fresh_var();
                body.push_str(&format!("size_t {} = {};\n", tmp, size));
                Ok((tmp, Type::Int { bits: 64, signed: false }))
            }
            ("len", Type::Array { size: None, .. }) => {
                let tmp = self.fresh_var();
                body.push_str(&format!("size_t {} = {}.len;\n", tmp, obj_var));
                Ok((tmp, Type::Int { bits: 64, signed: false }))
            }
            ("contain", _) | ("contains", _) | ("has", _) => {
                if !args.is_empty() {
                    let (item_var, _) = self.codegen_expr(&args[0], body) .check_error();
                    let tmp = self.fresh_var();
                    match &obj_ty {
                        Type::Array { size: Some(size), .. } => {
                            body.push_str(&format!("bool {} = false;\n", tmp));
                            body.push_str(&format!("for (int _i = 0; _i < {}; _i++) {{\n", size));
                            body.push_str(&format!("    if ({}[_i] == {}) {{ {} = true; break; }}\n", obj_var, item_var, tmp));
                            body.push_str("}\n");
                        }
                        Type::Array { size: None, .. } => {
                            body.push_str(&format!("bool {} = false;\n", tmp));
                            body.push_str(&format!("for (int _i = 0; _i < {}.len; _i++) {{\n", obj_var));
                            body.push_str(&format!("    if ({}.ptr[_i] == {}) {{ {} = true; break; }}\n", obj_var, item_var, tmp));
                            body.push_str("}\n");
                        }
                        _ => {
                            body.push_str(&format!("bool {} = false;\n", tmp));
                        }
                    }
                    Ok((tmp, Type::Bool))
                } else {
                    let tmp = self.fresh_var();
                    body.push_str(&format!("bool {} = false;\n", tmp));
                    Ok((tmp, Type::Bool))
                }
            }
 
            ("len", Type::Str { .. }) => {
                let tmp = self.fresh_var();
                body.push_str(&format!("size_t {} = strlen({});\n", tmp, obj_var));
                Ok((tmp, Type::Int { bits: 64, signed: false }))
            }
 
            _ => {
 
                if let Type::Struct { name } = &obj_ty {
                    let method_key = (name.clone(), method.to_string());
                    if let Some((_params, ret_ty)) = self.impl_methods.get(&method_key).cloned() { 
                        let mut arg_vars = vec![obj_var.clone()];
                        for arg in args {
                            let (var, _) = self.codegen_expr(arg, body) .check_error();
                            arg_vars.push(var);
                        }
                        let tmp = self.fresh_var();
                        let c_type = ret_ty.to_c_type(&self.arch);
                        let args_str = arg_vars.join(", ");
                        body.push_str(&format!("{} {} = {}_{}({});\n", c_type, tmp, name, method, args_str));
                        return Ok((tmp, ret_ty));
                    }
                }
                
 
                let mut arg_vars = vec![obj_var.clone()];
                for arg in args {
                    let (var, _) = self.codegen_expr(arg, body) .check_error();
                    arg_vars.push(var);
                }
                let tmp = self.fresh_var();
                let args_str = arg_vars.join(", ");
                body.push_str(&format!("int32_t {} = {}({});\n", tmp, method, args_str));
                Ok((tmp, Type::i32()))
            }
        }
    }

    pub fn codegen_module_call(&mut self, module: &str, func: &str, args: &[Expr], body: &mut String, _loc: SourceLocation) -> Result<(String, Type), ()> {
        let mut arg_vars = Vec::new();
        for arg in args {
            let (var, _) = self.codegen_expr(arg, body) .check_error();
            arg_vars.push(var);
        }
        
        let tmp = self.fresh_var();
        let args_str = arg_vars.join(", ");
        
 
        let method_key = (module.to_string(), func.to_string());
        if let Some((_, ret_ty)) = self.module_functions.get(&method_key).cloned() {
            let c_type = ret_ty.to_c_type(&self.arch);
            body.push_str(&format!("{} {} = {}_{}({});\n", c_type, tmp, module, func, args_str));
            return Ok((tmp, ret_ty));
        }
        
 
        body.push_str(&format!("int32_t {} = {}_{}({});\n", tmp, module, func, args_str));
        Ok((tmp, Type::i32()))
    }

    pub fn codegen_is_empty(&mut self, expr: &Expr, body: &mut String) -> Result<(String, Type), ()> {
        let (var, ty) = self.codegen_expr(expr, body) .check_error();
        let tmp = self.fresh_var();
        
        match &ty {
            Type::Array { size: Some(size), .. } => {
                body.push_str(&format!("bool {} = ({} == 0);\n", tmp, size));
            }
            Type::Array { size: None, .. } => {
                body.push_str(&format!("bool {} = ({}.len == 0);\n", tmp, var));
            }
            Type::Ptr(_) => {
                body.push_str(&format!("bool {} = ({} == NULL);\n", tmp, var));
            }
            Type::Str { .. } => {
                body.push_str(&format!("bool {} = (strlen({}) == 0);\n", tmp, var));
            }
            _ => {
                body.push_str(&format!("bool {} = ({} == 0);\n", tmp, var));
            }
        }
        
        Ok((tmp, Type::Bool))
    }

    pub fn codegen_filter(&mut self, obj: &Expr, reference: &Expr, body: &mut String) -> Result<(String, Type), ()> {
        let (obj_var, _obj_ty) = self.codegen_expr(obj, body) .check_error();
        let (_ref_var, _) = self.codegen_expr(reference, body) .check_error();
        let tmp = self.fresh_var();
    
        body.push_str(&format!("void* {} = {};\n", tmp, obj_var));
        Ok((tmp, Type::Ptr(Box::new(Type::Void))))
    }

    pub fn codegen_panic(&mut self, expr: &Expr, body: &mut String) -> Result<(String, Type), ()> {
        let (msg_var, _) = self.codegen_expr(expr, body) .check_error();
        let tmp = self.fresh_var();
        
        body.push_str(&format!("fprintf(stderr, \"panic: %s\\n\", {});\n", msg_var));
        body.push_str("exit(1);\n");
        body.push_str(&format!("int {} = 0;\n", tmp));
        
        Ok((tmp, Type::Void))
    }
}