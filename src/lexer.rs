use logos::{Lexer, Logos};
use std::ops::Range;

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token<'a> {
    #[regex(r"//[^\n\r]*", logos::skip)]
    Comment,

    #[regex(r#"c"([^"\\]|\\.)*""#, unescape_string)]
    CStringLiteral(String),

    #[regex(r#""([^"\\]|\\.)*""#, unescape_string)]
    StringLiteral(String),

    #[token("func")]
    Func,

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

    #[token("str")]
    Str,

    #[token("cstr")]
    CStr,

    #[token("for")]
    For,

    #[token(";")]
    Semicolon,

    #[token(":")]
    Colon,

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

    #[token("else")]
    Else,

    #[token("while")]
    While,

    #[token("and")]
    And,

    #[token("not")]
    Not,

    #[token("&")]
    Ampersand,

    #[token("or")]
    Or,

    #[token("as")]
    As,

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

    #[token("%")]
    Percent,

    #[token("+=")]
    AddAssign,

    #[token("-=")]
    SubAssign,

    #[token("*=")]
    MulAssign,

    #[token("/=")]
    DivAssign,

    // #[token("\n")]
    // Newline,
    #[token("var")]
    Var,

    #[token("const")]
    Const,

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

    #[regex(r#"[_a-zA-Z][_a-zA-Z0-9]*"#)]
    Ident(&'a str),

    // #[regex(r#"-?[0-9]+"#)]
    // IntLiteral(&'a str),
    #[regex(r#"-?[0-9]+"#, |lex| lex.slice().parse())]
    IntLit(i32),

    #[regex(r#"[0-9]+_u"#, parse_uint)]
    UIntLit(u32),

    #[regex(r#"-?[0-9]+_i8"#, parse_i8)]
    Int8Lit(i8),

    #[regex(r#"-?[0-9]+_i16"#, parse_i16)]
    Int16Lit(i16),

    #[regex(r#"-?[0-9]+_i32"#, parse_i32)]
    Int32Lit(i32),

    #[regex(r#"-?[0-9]+_i64"#, parse_i64)]
    Int64Lit(i64),

    #[regex(r#"[0-9]+_u8"#, parse_u8)]
    UInt8Lit(u8),

    #[regex(r#"[0-9]+_u16"#, parse_u16)]
    UInt16Lit(u16),

    #[regex(r#"[0-9]+_u32"#, parse_u32)]
    UInt32Lit(u32),

    #[regex(r#"[0-9]+_u64"#, parse_u64)]
    UInt64Lit(u64),

    #[regex(r"[ \t\r\n]+", logos::skip)]
    Whitespace,

    #[error]
    Error,
}

fn unescape_string<'a>(lex: &mut Lexer<'a, Token<'a>>) -> String {
    let mut full = lex.slice();
    if full.chars().nth(0) == Some('c') {
        full = &full[1..];
    }
    let without_quotes = &full[1..full.len() - 1];
    let s = without_quotes.to_owned();
    let mut chars = s.chars();

    let mut res = String::with_capacity(s.len());

    while let Some(c) = chars.next() {
        if c == '\\' {
            // Parse escaped character
            // TODO: Handle invalid escapes
            res.push(match chars.next().unwrap() {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '\\' => '\\',
                '"' => '"',
                other => panic!("Invalid string escape character: {}", other),
            });
        } else {
            res.push(c);
        }
    }

    res
}

fn parse_i8<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<i8, std::num::ParseIntError> {
    let slice = lex.slice();
    let slice = &slice[0..slice.len() - 3];
    slice.parse()
}

fn parse_i16<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<i16, std::num::ParseIntError> {
    let slice = lex.slice();
    let slice = &slice[0..slice.len() - 4];
    slice.parse()
}

fn parse_i32<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<i32, std::num::ParseIntError> {
    let slice = lex.slice();
    let slice = &slice[0..slice.len() - 4];
    slice.parse()
}

fn parse_i64<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<i64, std::num::ParseIntError> {
    let slice = lex.slice();
    let slice = &slice[0..slice.len() - 4];
    slice.parse()
}

fn parse_uint<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<u32, std::num::ParseIntError> {
    let slice = lex.slice();
    let slice = &slice[0..slice.len() - 2];
    slice.parse()
}

fn parse_u8<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<u8, std::num::ParseIntError> {
    let slice = lex.slice();
    let slice = &slice[0..slice.len() - 3];
    slice.parse()
}

fn parse_u16<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<u16, std::num::ParseIntError> {
    let slice = lex.slice();
    let slice = &slice[0..slice.len() - 4];
    slice.parse()
}

fn parse_u32<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<u32, std::num::ParseIntError> {
    let slice = lex.slice();
    let slice = &slice[0..slice.len() - 4];
    slice.parse()
}

fn parse_u64<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<u64, std::num::ParseIntError> {
    let slice = lex.slice();
    let slice = &slice[0..slice.len() - 4];
    slice.parse()
}

impl<'a> Token<'a> {
    pub fn to_lalr_triple(
        (t, r): (Token<'a>, Range<usize>),
    ) -> Result<(usize, Token, usize), Error> {
        if t == Token::Error {
            Err(Error {})
        } else {
            Ok((r.start, t, r.end))
        }
    }
}

#[derive(Debug)]
pub struct Error;
