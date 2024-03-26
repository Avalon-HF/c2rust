use std::collections::HashMap;
use std::fmt::Formatter;
use clang_ast::Node;
use crate::transpiler::Transpile;
use crate::types::Clang;

// #[derive(Debug)]
pub struct AST {
    node: Node<Clang>,
}

impl std::fmt::Debug for AST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let node = self.node.inner.iter().filter(|node| {
            match &node.kind {
                Clang::TypedefDecl(ty) => {
                    match ty.is_implicit {
                        Some(true) => false,
                        _ => true
                    }
                }
                Clang::Other => false,
                _ => true
            }
        }).collect::<Vec<_>>();
        f.debug_struct("AST")
            .field("node", &node)
            .finish()
    }
}

impl AST {
    /// 从 clang 命令行[^1]生成的 json 文本中构建语法树
    ///
    /// [1]: `clang++ -Xclang -ast-dump=json -fsyntax-only -nostdinc++ -nostdinc source.cpp`
    pub fn from_ast_json<S: AsRef<str>>(json_str: S) -> color_eyre::Result<Self> {
        let node = serde_json::from_str(json_str.as_ref())?;
        Ok(Self { node })
    }

    pub fn transpile(&self) -> color_eyre::Result<String> {
        let mut tokens = Vec::new();
        let ctx = NodeContext::init_from_ast(&self);
        for node in &self.node.inner {
            if let Some(token) = node.transpile(&ctx, &node.inner) {
                tokens.push(token);
            }
        }
        let rust_code = quote::quote! { #(#tokens)* }.to_string();

        Ok(rust_code)
    }
}

pub struct NodeContext<'a> {
    pub map: HashMap<u64, &'a Node<Clang>>,
}

impl<'a> NodeContext<'a> {
    pub fn init_from_ast(ast: &'a AST) -> Self {
        let mut map = HashMap::new();
        let mut stack = vec![&ast.node];
        while let Some(node) = stack.pop() {
            for child in &node.inner{
                stack.push(child);
            }
            map.insert(u64::from_str_radix(&node.id.to_string()[2..], 16).unwrap(), node);
        }
        Self { map }
    }
}