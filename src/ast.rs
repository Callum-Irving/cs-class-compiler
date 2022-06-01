pub struct Program(Vec<TopLevelStmt>);

pub enum TopLevelStmt {
    FunctionDef(FunctionDef),
    ConstDef(ConstDef),
}

pub struct FunctionDef {
    name: Ident,
    params: Vec<TypeBinding>,
    return_type: Option<Type>,
    body: BlockStmt,
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

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Literal(Literal),
    Ident(Ident),
}

#[derive(Debug)]
pub enum UnaryOp {
    Reference,
    Minus,
    Not,
}

#[derive(Debug)]
pub enum BinOp {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug)]
pub struct TypeBinding {
    pub name: Ident,
    pub value_type: Type,
}

#[derive(Debug)]
pub enum Type {
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
    Str,
}

#[derive(Debug)]
pub enum Literal {
    Int32(i32),
    Str(String),
}

#[derive(Debug)]
pub struct Ident(pub String);
