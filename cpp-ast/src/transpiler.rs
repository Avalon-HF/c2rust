use clang_ast::Node;
use proc_macro2::TokenStream;
use crate::ast::NodeContext;
use crate::types::Clang;

pub trait Transpile {
    fn transpile(&self, ctx: &NodeContext, inner: &[Node<Clang>]) -> Option<TokenStream>;
}

impl Transpile for Node<Clang> {
    fn transpile(&self, ctx: &NodeContext, inner: &[Node<Clang>]) -> Option<TokenStream> {
        match &self.kind {
            Clang::FunctionDecl(func) => func.transpile(ctx, inner),
            _ => None,
        }
    }
}