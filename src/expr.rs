struct Program(Vec<Decl>);

enum Decl {
    VarDecl(VarDecl),
    FunDecl(FunDecl),
}

enum VarDecl {
    Var,
    Const,
}

struct FunDecl {
    name: Ident,
    params: Vec<(Ident, Type)>,
    return_type: Option<Type>,
    body: Vec<Stmt>,
}

struct Ident(String);

enum Type {
    // Signed integers
    Int,
    Int8,
    Int16,
    Int32,
    Int64,

    // Unsigned integers
    UInt,
    UInt8,
    UInt16,
    UInt32,
    UInt64,

    Char,
    String,
    Bool,

    // TODO: Maybe remove this
    Void,
}

struct Stmt;
