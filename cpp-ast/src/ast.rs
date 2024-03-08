use std::fmt::Formatter;
use clang_ast::Node;
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
}