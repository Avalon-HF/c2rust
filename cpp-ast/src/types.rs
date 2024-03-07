
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Clang {
    /// typedef
    TypedefDecl(TypeDefDecl),
    /// nullptr 字面量
    CXXNullPtrLiteralExpr,
    /// 由 `{}` 块包围的一系列语句和声明
    CompoundStmt,
    /// 声明语句
    DeclStmt,
    /// 函数定义
    FunctionDecl(FunctionDecl),
    /// 函数参数定义
    ParmVarDecl,
    /// 函数的返回语句
    ReturnStmt,
    /// 其他 Kind 将被忽略
    Other,
}

#[derive(Debug, Deserialize)]
/// TODO: fields
pub struct TypeDefDecl;

/// 函数定义字段
#[derive(Debug, Deserialize)]
pub struct FunctionDecl {
    /// C++ 源码中对应的函数名
    pub name: String,
    /// 函数的 mangled name, 函数重载时区分同名函数
    pub mangled_name: String,
}