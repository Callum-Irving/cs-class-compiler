pub struct Program(Vec<TopLevelStmt>);

pub enum TopLevelStmt {
    ConstantDef,
    VariableDef,
    FunctionDef,
}

pub struct ConstantDef {
    binding: TypeBinding,
    value: Literal,
}

pub struct VariableDef {
    binding: TypeBinding,
    value: Literal,
}

pub struct FunctionDef {
    name: Identifier,
    params: Vec<TypeBinding>,
    return_type: Option<Type>,
}

pub struct TypeBinding {
    name: Identifier,
    value_type: Type,
}

pub enum Type {
    Int,
    UInt,
}

pub enum Literal {}

pub struct Identifier(String);
