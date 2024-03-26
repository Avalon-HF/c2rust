use clang_ast::Node;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use serde::Deserialize;
use syn::Expr;
use crate::ast::NodeContext;
use crate::transpiler::Transpile;
use crate::utils::ctype_to_rtype;

#[derive(Debug, Deserialize)]
pub enum Clang {
    /// typedef
    TypedefDecl(TypeDefDecl),
    /// nullptr 字面量
    CXXNullPtrLiteralExpr,
    /// 复合语句，由 `{}` 块包围的一系列语句和声明
    CompoundStmt(CompoundStmt),
    /// 声明语句
    DeclStmt(DeclStmt),
    /// 函数声明
    FunctionDecl(FunctionDecl),
    /// 函数参数声明
    ParmVarDecl(ParmValDecl),
    /// 函数的返回语句
    ReturnStmt,
    /// 二元操作符
    BinaryOperator(BinaryOperator),
    /// 引用一个声明
    DeclRefExpr(DeclRefExpr),
    /// 复合类型的声明
    CXXRecordDecl(CXXRecordDecl),
    /// 隐式类型转换
    ImplicitCastExpr(ImplicitCastExpr),
    /// 变量声明
    VarDecl(ValDecl),
    CStyleCastExpr(CStyleCastExpr),
    /// 注释块
    FullComment,
    /// 段落注释
    ParagraphComment,
    /// 文本注释，里面的内容是注释的文本
    TextComment(TextComment),
    /// 整型字面量
    IntegerLiteral(IntegerLiteral),
    /// 字符串字面量
    StringLiteral(StringLiteral),
    /// 其他 Kind 将被忽略
    Other,
}

#[derive(Debug, Deserialize)]
pub struct CompoundStmt;

#[derive(Debug, Deserialize)]
pub struct DeclStmt;

/// 类型定义字段
/// typedef <type.qual_type> <name>;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefDecl {
    /// typedef 后的名称
    pub name: String,
    /// 原始类型
    pub r#type: Type,
    /// 是否是编译器自动生成的
    /// 自动生成的类型定义对转译 rust 没啥影响，可以忽略
    pub is_implicit: Option<bool>,
}

/// 函数声明字段
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDecl {
    /// C++ 源码中对应的函数名
    pub name: String,
    /// 函数的 mangled name, 函数重载时区分同名函数
    pub mangled_name: Option<String>,
    /// 函数类型，如 int (int, int)
    pub r#type: Type,
    /// 函数在源码中的位置信息
    pub loc: Option<Loc>,
}

/// 函数参数声明字段
#[derive(Debug, Deserialize)]
pub struct ParmValDecl {
    /// 参数名
    pub name: Option<String>,
    /// 参数类型
    pub r#type: Type,
}

/// 二元操作符字段
#[derive(Debug, Deserialize)]
pub struct BinaryOperator {
    /// 操作符
    pub opcode: String,
    /// 结果类型
    pub r#type: Type,
}

/// 对 AST 中某个节点的引用
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeclRefExpr {
    pub referenced_decl: ReferencedDecl,
    /// 引用的声明类型
    pub r#type: Type,
}

/// 复合类型的声明字段
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CXXRecordDecl {
    /// 结构体/类名
    pub name: Option<String>,
    /// class, struct 等
    pub tag_used: String,
    /// 是否是抽象类
    pub is_abstract: Option<bool>,
    pub is_polymorphic: Option<bool>,
    pub loc: Loc,
}

/// 隐式类型转换字段
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplicitCastExpr {
    /// 结果类型
    pub r#type: Type,
    /// 转换的种类
    pub cast_kind: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CStyleCastExpr {
    pub cast_kind: String,
    pub r#type: Type,
}

/// 变量声明字段
#[derive(Debug, Deserialize)]
pub struct ValDecl {
    /// 变量名
    pub name: String,
    pub r#type: Type,
    /// 是否马上初始化 (有初始值)
    pub init: Option<String>,
}

/// 注释字段
#[derive(Debug, Deserialize)]
pub struct TextComment {
    pub text: String,
}

/// 整型字面量字段
#[derive(Debug, Deserialize)]
pub struct IntegerLiteral {
    /// 整型字面量的值
    pub value: String,
    /// 整型字面量的类型
    pub r#type: Type,
}

/// 字符串字面量字段
#[derive(Debug, Deserialize)]
pub struct StringLiteral {
    /// 字符串字面量的值
    pub value: String,
    /// 字符串字面量的类型，如 char[5]
    pub r#type: Type,
}

/// 类型字段
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    /// C++ 源码中对应的类型名
    pub qual_type: String,
    /// 实际类型名
    pub desugared_qual_type: Option<String>,
}

impl Type {
    pub fn type_name(&self) -> &str {
        self.desugared_qual_type.as_ref().unwrap_or(&self.qual_type)
    }

    pub fn return_type(&self) -> Option<&str> {
        self.qual_type.splitn(2, " (").next()
    }
}

/// 在源码中的位置信息
#[derive(Debug, Deserialize)]
pub struct Loc {
    /// 来自于哪个文件
    pub file: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReferencedDecl {
    pub id: String,
}

//////////////////////////////////

impl Transpile for CompoundStmt {
    fn transpile(&self, ctx: &NodeContext, inner: &[Node<Clang>]) -> Option<TokenStream> {
        let mut tokens = Vec::new();
        for inner in inner {
            let code = match &inner.kind {
                Clang::DeclStmt(decl_stmt) => decl_stmt.transpile(ctx, &inner.inner),
                _ => None
            };
            if let Some(token) = code {
                tokens.push(token);
            }
        }
        Some(quote! { #(#tokens)* })
    }
}

impl Transpile for DeclStmt {
    fn transpile(&self, ctx: &NodeContext, inner: &[Node<Clang>]) -> Option<TokenStream> {
        let mut tokens = Vec::new();
        for inner in inner {
            let code = match &inner.kind {
                Clang::VarDecl(val_decl) => val_decl.transpile(ctx, &inner.inner),
                _ => None
            };
            if let Some(token) = code {
                tokens.push(token);
            }
        }
        Some(quote! { #(#tokens)* })
    }
}

impl Transpile for ValDecl {
    fn transpile(&self, ctx: &NodeContext, inner: &[Node<Clang>]) -> Option<TokenStream> {
        let name = &self.name;
        let ty = self.r#type.type_name();
        let (ty, is_pointer) = ctype_to_rtype(ty);

        let name = syn::parse_str::<Ident>(name).unwrap();
        let ty = syn::parse_str::<syn::Type>(&ty).unwrap();

        let ty = if is_pointer {
            quote! { Box<#ty> }
        } else {
            quote! { #ty }
        };

        if self.init.is_some() {
            let mut init_tokens = Vec::new();
            for inner in inner {
                match &inner.kind {
                    // TODO: 没时间了，这里应该一层层往下找，目前看到 CStyleCastExpr 默认是 malloc 操作
                    Clang::CStyleCastExpr(_) => {
                        init_tokens.push(quote! { Box::new(Default::default()); })
                    }
                    Clang::IntegerLiteral(int_literal) => {
                        let value = &int_literal.value;
                        let value = syn::parse_str::<Expr>(value).unwrap();
                        init_tokens.push(quote! { #value; })
                    }
                    _ => {}
                }
            }
            Some(quote! { let #name: #ty = #(#init_tokens)*; })
        } else {
            Some(quote! { let #name: #ty; })
        }
    }
}

impl Transpile for FunctionDecl {
    fn transpile(&self, ctx: &NodeContext, inner: &[Node<Clang>]) -> Option<TokenStream> {
        let return_type = self.r#type.return_type().unwrap();
        let mut return_type = ctype_to_rtype(return_type).0;
        let name = self.name.as_str();

        if name == "main" {
            return_type = "()".to_string();
        }

        let mut stmt = Vec::new();
        for inner in inner {
            match &inner.kind {
                Clang::CompoundStmt(compound_stmt) => {
                    stmt.push(compound_stmt.transpile(ctx, &inner.inner).unwrap());
                }
                Clang::BinaryOperator(bin_operator) => {
                    let opcode = &bin_operator.opcode;
                }
                _ => {}
            }
        }

        let name = syn::parse_str::<Ident>(name).unwrap();
        let ty = syn::parse_str::<syn::Type>(&return_type).unwrap();

        let fn_decl = quote! {
            fn #name() -> #ty {
                #(#stmt)*
            }
        };
        Some(fn_decl)
    }
}