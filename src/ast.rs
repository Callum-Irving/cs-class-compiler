pub struct Program(Vec<TopLevelStmt>);

pub enum TopLevelStmt {
    ConstantDef,
    VariableDef,
    FunctionDef,
}

pub struct ConstDef {
    pub binding: TypeBinding,
    pub value: Expr,
}

pub struct VarDef {
    pub binding: TypeBinding,
    pub value: Expr,
}

pub struct FunctionDef {
    name: Ident,
    params: Vec<TypeBinding>,
    return_type: Option<Type>,
}

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Literal(Literal),
    Ident(Ident),
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
