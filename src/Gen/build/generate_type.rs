use crate::import::*;

impl Codegen {
 
    pub fn codegen_string(&mut self, s: &str, body: &mut String) -> (String, Type) {
        let tmp = self.fresh_var();
        let escaped = s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n").replace('\t', "\\t");
        body.push_str(&format!("const char* {} = \"{}\";\n", tmp, escaped));
        (tmp, Type::Str { len_type: Box::new(Type::i64()) })
    }

    pub fn codegen_number(&mut self, n: i32, body: &mut String) -> (String, Type) {
        let tmp = self.fresh_var();
        body.push_str(&format!("int32_t {} = {};\n", tmp, n));
        (tmp, Type::i32())
    }

    pub fn codegen_float(&mut self, f: f32, body: &mut String) -> (String, Type) {
        let tmp = self.fresh_var();
        body.push_str(&format!("float {} = {};\n", tmp, f));
        (tmp, Type::f32())
    }

    pub fn codegen_bool(&mut self, b: bool, body: &mut String) -> (String, Type) {
        let tmp = self.fresh_var();
        body.push_str(&format!("bool {} = {};\n", tmp, if b { "true" } else { "false" }));
        (tmp, Type::Bool)
    }

    pub fn codegen_char(&mut self, c: i32, body: &mut String) -> (String, Type) {
        let tmp = self.fresh_var();
        body.push_str(&format!("char {} = {};\n", tmp, c));
        (tmp, Type::char8())
    }

    pub fn codegen_hex_number(&mut self, n: i32, body: &mut String) -> (String, Type) {
        let tmp = self.fresh_var();
        body.push_str(&format!("int32_t {} = 0x{:x};\n", tmp, n));
        (tmp, Type::i32())
    }

    pub fn codegen_binary_number(&mut self, n: i32, body: &mut String) -> (String, Type) {
        let tmp = self.fresh_var();
        body.push_str(&format!("int32_t {} = 0b{:b};\n", tmp, n));
        (tmp, Type::i32())
    }

    pub fn codegen_octal_number(&mut self, n: i32, body: &mut String) -> (String, Type) {
        let tmp = self.fresh_var();
        body.push_str(&format!("int32_t {} = 0{:o};\n", tmp, n));
        (tmp, Type::i32())
    }
}   