
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Clang {
    /// typedef
    TypedefDecl(TypeDefDecl),
    /// nullptr 字面量
    CXXNullPtrLiteralExpr,
    /// 复合语句，由 `{}` 块包围的一系列语句和声明
    CompoundStmt,
    /// 声明语句
    DeclStmt,
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
    pub loc: Loc,
}

/// 函数参数声明字段
#[derive(Debug, Deserialize)]
pub struct ParmValDecl {
    /// 参数名
    pub name: String,
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
    pub name: String,
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