use clang_ast::Node;
use crate::types::Clang;

#[derive(Debug)]
pub struct AST {
    node: Node<Clang>,
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