use num_bigint::BigInt;

pub struct Program(pub Vec<TopLevelStmt>);

pub enum TopLevelStmt {
    FunctionDef(FunctionDef),
    ExternDef(ExternDef),
    ConstDef(GlobalConstDef),
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

pub enum Stmt {
    ExprStmt(Expr),
    BlockStmt(BlockStmt),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
    ConstDef(ConstDef),
    VarDef(VarDef),
    ReturnStmt(Expr),
}

pub struct BlockStmt {
    pub inners: Vec<Stmt>,
}

pub struct IfStmt {
    pub condition: Expr,
    pub body: BlockStmt,
}

pub struct WhileStmt {
    pub condition: Expr,
    pub body: BlockStmt,
}

pub struct ConstDef {
    pub binding: TypeBinding,
    pub value: Expr,
}

pub struct VarDef {
    pub binding: TypeBinding,
    pub value: Expr,
}

// EXPRESSIONS

pub struct Expr {
    pub ty: Type,
    pub val: ExprInner,
}

pub enum ExprInner {
    FunctionCall(FunctionCall),
    IndexExpr(IndexExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Array(ArrayExpr),
    Cast(CastExpr),
    Literal(Literal),
    Ident(String),
}

pub struct FunctionCall {
    pub name: Box<Expr>,
    pub args: Vec<Expr>,
}

pub struct IndexExpr {
    pub name: Box<Expr>,
    pub index: Box<Expr>,
}

pub struct BinaryExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
    pub op: BinOp,
}

pub struct UnaryExpr {
    pub data: Box<Expr>,
    pub op: UnaryOp,
}

pub struct ArrayExpr {
    pub items: Vec<Expr>,
    pub len: usize,
}

pub struct CastExpr {
    pub original: Box<Expr>,
    pub to_type: Type,
}

pub enum BinOp {
    Plus,
    Minus,
    Times,
    Divide,
    LogicalAnd,
    LogicalOr,
}

pub enum UnaryOp {
    Reference,
    Minus,
    Not,
}

// LITERALS

pub struct Literal {
    pub ty: Type,
    pub val: LiteralInner,
}

pub enum LiteralInner {
    Int(BigInt),
    Str(String),
    Bool(bool),
}

pub struct TypeBinding {
    pub name: String,
    pub ty: Type,
}

pub enum Type {
    Array(Box<Type>),
    Ref(Box<Type>),
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Char,
    Str,
    Bool,
    NoneType,
}
