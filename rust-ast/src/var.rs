pub struct Var {
    name: String,
    ty: Type,
}

pub struct Type {
    name: String,
    is_pointer: bool,
    is_mut: bool,
}