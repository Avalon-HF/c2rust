use cpp_ast::ast::AST;

fn main() {
    let ast_json = std::fs::read_to_string("ast.json").unwrap();
    let cpp_ast = AST::from_ast_json(ast_json).unwrap();

    dbg!(cpp_ast);

    let rust_code = quote::quote! {
        fn main() {
            println!("Hello, world!");
        }
    };

    println!("{}", rust_code.to_string());
}
