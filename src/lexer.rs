use logos::Logos;

#[derive(Logos, Debug)]
pub enum Token<'a> {
    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral(&'a str),

    #[token("void")]
    Void,

    #[token("float")]
    Float,

    #[token("double")]
    Double,

    #[token("bool")]
    Bool,

    #[token("char")]
    Char,

    #[token("uint")]
    UInt,

    #[token("uint8")]
    UInt8,

    #[token("uint16")]
    UInt16,

    #[token("uint32")]
    UInt32,

    #[token("uint64")]
    UInt64,

    #[token("int")]
    Int,

    #[token("int8")]
    Int8,

    #[token("int16")]
    Int16,

    #[token("int32")]
    Int32,

    #[token("int64")]
    Int64,

    #[token("for")]
    For,

    #[token(";")]
    Semicolon,

    #[token("==")]
    IsEqual,

    #[token("!=")]
    NotEqual,

    #[token(">=")]
    Gte,

    #[token("<=")]
    Lte,

    #[token(">")]
    Gt,

    #[token("<")]
    Lt,

    #[token("if")]
    If,

    #[token("and")]
    And,

    #[token("not")]
    Not,

    #[token("or")]
    Or,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Times,

    #[token("/")]
    Divide,

    #[token("+=")]
    AddAssign,

    #[token("-=")]
    SubAssign,

    #[token("*=")]
    MulAssign,

    #[token("/=")]
    DivAssign,

    #[token("\n")]
    Newline,

    #[token("var")]
    Var,

    #[token("in")]
    In,

    #[token("fun")]
    Fun,

    #[token("->")]
    Arrow,

    #[token(",")]
    Comma,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LCurly,

    #[token("}")]
    RCurly,

    #[token("[")]
    LSquare,

    #[token("]")]
    RSquare,

    #[token("=")]
    Equals,

    #[token("return")]
    Return,

    #[token("extern")]
    Extern,

    #[regex(r#"[_a-zA-Z][_a-zA-Z0-9]+"#)]
    Ident(&'a str),

    #[regex(r#"[_a-zA-Z][_a-zA-Z0-9]+!"#)]
    Builtin(&'a str),

    #[regex(r#"-?[0-9]+"#)]
    IntLiteral(&'a str),

    #[regex(r"[ \t\r]+", logos::skip)]
    Whitespace,

    #[error]
    Error,
}
