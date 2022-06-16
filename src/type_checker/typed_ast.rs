use std::collections::HashMap;

pub struct Program(pub Vec<TopLevelStmt>);

pub enum TopLevelStmt {
    ClassDef(ClassDef),
    FunctionDef(FunctionDef),
    ExternDef(ExternDef),
    ConstDef(GlobalConstDef),
}

#[derive(Clone)]
pub struct ClassDef {
    pub name: String,
    pub fields: Vec<(String, Type)>,
}

// TOP LEVEL STATEMENTS

pub struct FunctionDef {
    pub name: String,
    pub params: Vec<TypeBinding>,
    pub return_type: Type,
    pub body: BlockStmt,
}

pub struct ExternDef {
    pub name: String,
    pub params: Vec<TypeBinding>,
    pub return_type: Type,
}

pub struct GlobalConstDef {
    pub binding: TypeBinding,
    pub value: Literal,
}

// STATEMENTS

#[derive(Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    BlockStmt(BlockStmt),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
    ConstDef(ConstDef),
    VarDef(VarDef),
    ReturnStmt(Expr),
}

#[derive(Clone)]
pub struct BlockStmt {
    pub inners: Vec<Stmt>,
}

#[derive(Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub body: BlockStmt,
    pub else_stmt: Option<IfOrElse>,
}

#[derive(Clone)]
pub enum IfOrElse {
    If(Box<IfStmt>),
    Else(BlockStmt),
}

#[derive(Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: BlockStmt,
}

#[derive(Clone)]
pub struct ConstDef {
    pub binding: TypeBinding,
    pub value: Expr,
}

#[derive(Clone)]
pub struct VarDef {
    pub binding: TypeBinding,
    pub value: Expr,
}

// EXPRESSIONS

#[derive(Clone)]
pub struct Expr {
    pub ty: Type,
    pub val: ExprInner,
}

#[derive(Clone)]
pub enum ExprInner {
    Class(ClassExpr),
    FunctionCall(FunctionCall),
    IndexExpr(IndexExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Array(ArrayExpr),
    Cast(CastExpr),
    Literal(Literal),
    Ident(String),
}

#[derive(Clone)]
pub struct ClassExpr {
    pub class: String,
    pub fields: Vec<(String, Box<Expr>)>,
}

#[derive(Clone)]
pub struct FunctionCall {
    pub name: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Clone)]
pub struct IndexExpr {
    pub name: Box<Expr>,
    pub index: Box<Expr>,
}

#[derive(Clone)]
pub struct BinaryExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
    pub op: BinOp,
}

#[derive(Clone)]
pub struct UnaryExpr {
    pub data: Box<Expr>,
    pub op: UnaryOp,
}

#[derive(Clone)]
pub struct ArrayExpr {
    pub items: Vec<Expr>,
    pub len: usize,
}

#[derive(Clone)]
pub struct CastExpr {
    pub original: Box<Expr>,
    pub to_type: Type,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum UnaryOp {
    Reference,
    Deref,
    Minus,
    Not,
}

// LITERALS

#[derive(Clone)]
pub struct Literal {
    pub ty: Type,
    pub val: LiteralInner,
}

#[derive(Clone)]
pub enum LiteralInner {
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

#[derive(Clone)]
pub struct TypeBinding {
    pub name: String,
    pub ty: Type,
}

#[derive(Clone)]
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
    NoneType,
}
