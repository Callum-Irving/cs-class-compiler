use std::collections::HashMap;

pub struct Program(pub Vec<TopLevelStmt>);

pub enum TopLevelStmt {
    ClassDef(ClassDef),
    FunctionDef(FunctionDef),
    ConstDef(GlobalConstDef),
    ExternDef(ExternDef),
}

pub struct ClassDef {
    pub name: String,
    pub fields: Vec<(String, Type)>,
}

pub struct FunctionDef {
    pub name: String,
    pub params: Vec<TypeBinding>,
    pub return_type: Option<Type>,
    pub body: BlockStmt,
}

pub struct ExternDef {
    pub name: String,
    pub params: Vec<TypeBinding>,
    pub return_type: Option<Type>,
}

pub struct GlobalConstDef {
    pub binding: TypeBinding,
    pub value: Literal,
}

// STATEMENTS

#[derive(Debug)]
pub enum Stmt {
    ExprStmt(Expr),
    BlockStmt(BlockStmt),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
    ConstDef(ConstDef),
    VarDef(VarDef),
    ReturnStmt(Expr),
}

#[derive(Debug)]
pub struct BlockStmt(pub Vec<Stmt>);

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub body: BlockStmt,
    pub else_stmt: Option<IfOrElse>,
}

#[derive(Debug)]
pub enum IfOrElse {
    If(Box<IfStmt>),
    Else(BlockStmt),
}

#[derive(Debug)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: BlockStmt,
}

#[derive(Debug)]
pub struct ConstDef {
    pub binding: TypeBinding,
    pub value: Expr,
}

#[derive(Debug)]
pub struct VarDef {
    pub binding: TypeBinding,
    pub value: Expr,
}

// EXPRESSIONS

#[derive(Debug, Clone)]
pub enum Expr {
    Class(ClassExpr),
    FunctionCall(FunctionCall),
    IndexExpr(Box<Expr>, Box<Expr>),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Array(Vec<Expr>, usize),
    Cast(Box<Expr>, Type),
    Literal(Literal),
    Ident(String),
}

#[derive(Debug, Clone)]
pub struct ClassExpr {
    pub class: String,
    pub fields: Vec<(String, Box<Expr>)>,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Reference,
    Deref,
    Minus,
    Not,
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Plus,
    Minus,
    Times,
    Divide,
    LogicalAnd,
    LogicalOr,
    Equals,
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
}

#[derive(Debug, Clone)]
pub struct TypeBinding {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum Type {
    Class(String),
    Array(Box<Type>, usize),
    Ref(Box<Type>),
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Char,
    Str,
    CStr,
    Bool,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i32),
    UInt(u32),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Str(String),
    CStr(String),
    Bool(bool),
}
