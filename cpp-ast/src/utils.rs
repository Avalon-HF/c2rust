/// return: (rust_type, is_pointer)
pub fn ctype_to_rtype(ctype: &str) -> (String, bool) {
    match ctype {
        "int" => ("i32".to_string(), false),
        "char" => ("i8".to_string(), false),
        "void" => ("()".to_string(), false),
        "int *" => ("i32".to_string(), true),
        _ => (ctype.to_string(), false),
    }
}