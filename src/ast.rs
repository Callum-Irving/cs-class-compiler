pub struct Program(Vec<TopLevelStmt>);

pub enum TopLevelStmt {
    ConstantDef,
    VariableDef,
    FunctionDef,
}

pub struct ConstDef {
    pub binding: TypeBinding,
    pub value: i32,
}

pub struct VarDef {
    pub binding: TypeBinding,
    pub value: i32,
}

pub struct FunctionDef {
    name: Ident,
    params: Vec<TypeBinding>,
    return_type: Option<Type>,
}

#[derive(Debug)]
pub struct TypeBinding {
    pub name: Ident,
    pub value_type: Type,
}

#[derive(Debug)]
pub enum Type {
    Int,
    UInt,
}

pub enum Literal {}

#[derive(Debug)]
pub struct Ident(pub String);
