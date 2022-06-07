use num_bigint::BigInt;

pub struct Program(pub Vec<TopLevelStmt>);

pub enum TopLevelStmt {
    FunctionDef(FunctionDef),
    ConstDef(ConstDef),
    ExternDef(ExternDef),
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
pub struct Expr {
    pub ty: Option<Type>,
    pub val: ExprInner,
}

impl Expr {
    pub fn untyped(val: ExprInner) -> Expr {
        Self { ty: None, val }
    }

    pub fn with_type(ty: Type, val: ExprInner) -> Expr {
        Self { ty: Some(ty), val }
    }
}

#[derive(Debug, Clone)]
pub enum ExprInner {
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
pub struct FunctionCall {
    pub name: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Reference,
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
}

#[derive(Debug, Clone)]
pub struct TypeBinding {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum Type {
    Array(Box<Type>),
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
    Bool,
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub ty: Option<Type>,
    pub val: LiteralInner,
}

#[derive(Debug, Clone)]
pub enum LiteralInner {
    Int(BigInt),
    Str(String),
    Bool(bool),
}

impl Literal {
    pub fn untyped(val: LiteralInner) -> Literal {
        Self { ty: None, val }
    }

    pub fn with_type(ty: Type, val: LiteralInner) -> Literal {
        Self { ty: Some(ty), val }
    }
}
