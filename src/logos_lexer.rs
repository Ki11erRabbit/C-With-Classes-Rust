use logos::{Logos, Lexer};

use std::fmt;


#[derive(Logos, Debug, PartialEq)]
pub enum TokenPreparse<'input> {

    #[token("\n")]
    Newline,
    #[regex(" ")]
    Space,
    #[token("\t")]
    Tab,
    #[token("\r\n")]
    CarriageReturnNewline,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice())]
    Word(&'input str),

    #[regex("(0[xX][0-9a-fA-F]+)|[0-9]+", |lex| lex.slice())]
    Number(&'input str),

    //#[regex("\"[^\"]*\"", |lex| lex.slice())]
    //String(&'input str),
    #[token("\"")]
    StringDelimiter,
    
    #[regex("'[^']'", |lex| lex.slice())]
    Character(&'input str),
    #[regex(r#""/\*~(.*\*/.*)\*/""#, |lex| lex.slice())]
    Comment(&'input str),
    #[token(";")]
    SemiColon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Period,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulo,
    #[token("==")]
    Equals,
    #[token("!=")]
    NotEquals,
    #[token("<")]
    LessThan,
    #[token("<=")]
    LessThanOrEqual,
    #[token(">")]
    GreaterThan,
    #[token(">=")]
    GreaterThanOrEqual,
    #[token("&&")]
    LogicalAnd,
    #[token("||")]
    LogicalOr,
    #[token("!")]
    LogicalNot,
    #[token("&")]
    BitwiseAnd,
    #[token("|")]
    BitwiseOr,
    #[token("~")]
    BitwiseNot,
    #[token("^")]
    BitwiseXor,
    #[token("<<")]
    BitwiseLeftShift,
    #[token(">>")]
    BitwiseRightShift,
    #[token("=")]
    Assignment,
    #[token("+=")]
    PlusEquals,
    #[token("-=")]
    MinusEquals,
    #[token("*=")]
    StarEquals,
    #[token("/=")]
    DivideEquals,
    #[token("%=")]
    ModuloEquals,
    #[token("&=")]
    BitwiseAndEquals,
    #[token("|=")]
    BitwiseOrEquals,
    #[token("^=")]
    BitwiseXorEquals,
    #[token("<<=")]
    BitwiseLeftShiftEquals,
    #[token(">>=")]
    BitwiseRightShiftEquals,
    #[token("++")]
    Increment,
    #[token("--")]
    Decrement,
    #[token("->")]
    Arrow,
    #[token("?")]
    QuestionMark,
    #[token("\\")]
    Backslash,
    #[token("#")]
    Hash,
    #[token("auto")]
    Auto,
    #[token("double")]
    Double,
    #[token("int")]
    Int,
    #[token("struct")]
    Struct,
    #[token("break")]
    Break,
    #[token("else")]
    Else,
    #[token("long")]
    Long,
    #[token("switch")]
    Switch,
    #[token("case")]
    Case,
    #[token("enum")]
    Enum,
    #[token("register")]
    Register,
    #[token("typedef")]
    Typedef,
    #[token("char")]
    Char,
    #[token("extern")]
    Extern,
    #[token("return")]
    Return,
    #[token("union")]
    Union,
    #[token("continue")]
    Continue,
    #[token("for")]
    For,
    #[token("signed")]
    Signed,
    #[token("void")]
    Void,
    #[token("do")]
    Do,
    #[token("if")]
    If,
    #[token("static")]
    Static,
    #[token("while")]
    While,
    #[token("default")]
    Default,
    #[token("goto")]
    Goto,
    #[token("sizeof")]
    Sizeof,
    #[token("volatile")]
    Volatile,
    #[token("const")]
    Const,
    #[token("float")]
    Float,
    #[token("short")]
    Short,
    #[token("unsigned")]
    Unsigned,
    
    //todo check some of these for correctness
    #[token("restrict")]
    Restrict,
    #[token("inline")]
    Inline,
    #[token("alignas")]
    Alignas,
    #[token("alignof")]
    Alignof,
    #[token("atomic")]
    Atomic,
    #[token("bool")]
    Bool,
    #[token("complex")]
    Complex,
    #[token("_Generic")]
    Generic,
    #[token("imaginary")]
    Imaginary,
    #[token("noreturn")]
    Noreturn,
    #[token("static_assert")]
    StaticAssert,
    #[token("thread_local")]
    ThreadLocal,
    #[token("constexpr")]
    ConstExpr,
    #[token("nullptr")]
    Nullptr,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("typeof")]
    Typeof,
    #[token("typeofunqual")]
    TypeofUnqual,


    #[token("private")]
    Private,
    #[token("class")]
    Class,
}


pub enum Token {

    Newline,
    Word(String),
    Number(String),
    String(String),
    Include(String),
    Macro(String),
    Preprocessor(String),
    Character(String),
    Comment(String),
    SemiColon,
    Colon,
    Comma,
    Period,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Plus,
    Minus,
    Star,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
    Assignment,
    PlusEquals,
    MinusEquals,
    StarEquals,
    DivideEquals,
    ModuloEquals,
    BitwiseAndEquals,
    BitwiseOrEquals,
    BitwiseXorEquals,
    BitwiseLeftShiftEquals,
    BitwiseRightShiftEquals,
    Increment,
    Decrement,
    Arrow,
    QuestionMark,
    Backslash,
    Hash,
    Auto,
    Double,
    Int,
    Struct,
    Break,
    Else,
    Long,
    Switch,
    Case,
    Enum,
    Register,
    Typedef,
    Char,
    Extern,
    Return,
    Union,
    Continue,
    For,
    Signed,
    Void,
    Do,
    If,
    Static,
    While,
    Default,
    Goto,
    Sizeof,
    Volatile,
    Const,
    Float,
    Short,
    Unsigned,
    
    //todo check some of these for correctness
    Restrict,
    Inline,
    Alignas,
    Alignof,
    Atomic,
    Bool,
    Complex,
    Generic,
    Imaginary,
    Noreturn,
    StaticAssert,
    ThreadLocal,
    ConstExpr,
    Nullptr,
    True,
    False,
    Typeof,
    TypeofUnqual,


    Private,
    Class,
}


#[derive(Debug)]
pub enum LexerError {
    UnterminatedString,
    UnrecognizedToken,
}


pub fn parse<'input>(input: &'input str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();
    
    let mut lexer = TokenPreparse::lexer(input);
    while let Some(Ok(token)) = lexer.next() {
        tokens.push(match token {
            TokenPreparse::StringDelimiter => {
                in_string(&mut lexer)?
            },
            TokenPreparse::Newline => Token::Newline,
            TokenPreparse::Space => continue,
            TokenPreparse::Tab => continue,
            TokenPreparse::Backslash => Token::Backslash,
            TokenPreparse::Hash => {
                in_preprocessor(&mut lexer)?
            },
            TokenPreparse::CarriageReturnNewline => Token::Newline,
            TokenPreparse::Character(character) => Token::Character(character.to_string()),
            TokenPreparse::Comment(comment) => Token::Comment(comment.to_string()),
            TokenPreparse::SemiColon => Token::SemiColon,
            TokenPreparse::Colon => Token::Colon,
            TokenPreparse::Comma => Token::Comma,
            TokenPreparse::Period => Token::Period,
            TokenPreparse::LeftParen => Token::LeftParen,
            TokenPreparse::RightParen => Token::RightParen,
            TokenPreparse::LeftBrace => Token::LeftBrace,
            TokenPreparse::RightBrace => Token::RightBrace,
            TokenPreparse::LeftBracket => Token::LeftBracket,
            TokenPreparse::RightBracket => Token::RightBracket,
            TokenPreparse::Plus => Token::Plus,
            TokenPreparse::Minus => Token::Minus,
            TokenPreparse::Star => Token::Star,
            TokenPreparse::Divide => Token::Divide,
            TokenPreparse::Modulo => Token::Modulo,
            TokenPreparse::Equals => Token::Equals,
            TokenPreparse::NotEquals => Token::NotEquals,
            TokenPreparse::LessThan => Token::LessThan,
            TokenPreparse::LessThanOrEqual => Token::LessThanOrEqual,
            TokenPreparse::GreaterThan => Token::GreaterThan,
            TokenPreparse::GreaterThanOrEqual => Token::GreaterThanOrEqual,
            TokenPreparse::LogicalAnd => Token::LogicalAnd,
            TokenPreparse::LogicalOr => Token::LogicalOr,
            TokenPreparse::LogicalNot => Token::LogicalNot,
            TokenPreparse::BitwiseAnd => Token::BitwiseAnd,
            TokenPreparse::BitwiseOr => Token::BitwiseOr,
            TokenPreparse::BitwiseNot => Token::BitwiseNot,
            TokenPreparse::BitwiseXor => Token::BitwiseXor,
            TokenPreparse::BitwiseLeftShift => Token::BitwiseLeftShift,
            TokenPreparse::BitwiseRightShift => Token::BitwiseRightShift,
            TokenPreparse::Assignment => Token::Assignment,
            TokenPreparse::PlusEquals => Token::PlusEquals,
            TokenPreparse::MinusEquals => Token::MinusEquals,
            TokenPreparse::StarEquals => Token::StarEquals,
            TokenPreparse::DivideEquals => Token::DivideEquals,
            TokenPreparse::ModuloEquals => Token::ModuloEquals,
            TokenPreparse::BitwiseAndEquals => Token::BitwiseAndEquals,
            TokenPreparse::BitwiseOrEquals => Token::BitwiseOrEquals,
            TokenPreparse::BitwiseXorEquals => Token::BitwiseXorEquals,
            TokenPreparse::BitwiseLeftShiftEquals => Token::BitwiseLeftShiftEquals,
            TokenPreparse::BitwiseRightShiftEquals => Token::BitwiseRightShiftEquals,
            TokenPreparse::Increment => Token::Increment,
            TokenPreparse::Decrement => Token::Decrement,
            TokenPreparse::Arrow => Token::Arrow,
            TokenPreparse::QuestionMark => Token::QuestionMark,
            TokenPreparse::Auto => Token::Auto,
            TokenPreparse::Double => Token::Double,
            TokenPreparse::Int => Token::Int,
            TokenPreparse::Struct => Token::Struct,
            TokenPreparse::Break => Token::Break,
            TokenPreparse::Else => Token::Else,
            TokenPreparse::Long => Token::Long,
            TokenPreparse::Switch => Token::Switch,
            TokenPreparse::Case => Token::Case,
            TokenPreparse::Enum => Token::Enum,
            TokenPreparse::Register => Token::Register,
            TokenPreparse::Typedef => Token::Typedef,
            TokenPreparse::Char => Token::Char,
            TokenPreparse::Extern => Token::Extern,
            TokenPreparse::Return => Token::Return,
            TokenPreparse::Union => Token::Union,
            TokenPreparse::Continue => Token::Continue,
            TokenPreparse::For => Token::For,
            TokenPreparse::Signed => Token::Signed,
            TokenPreparse::Void => Token::Void,
            TokenPreparse::Default => Token::Default,
            TokenPreparse::Goto => Token::Goto,
            TokenPreparse::Sizeof => Token::Sizeof,
            TokenPreparse::Volatile => Token::Volatile,
            TokenPreparse::Do => Token::Do,
            TokenPreparse::If => Token::If,
            TokenPreparse::Short => Token::Short,
            TokenPreparse::Unsigned => Token::Unsigned,
            TokenPreparse::While => Token::While,
            TokenPreparse::Default => Token::Default,
            TokenPreparse::Static => Token::Static,
            TokenPreparse::Const => Token::Const,
            TokenPreparse::Float => Token::Float,
            TokenPreparse::Restrict => Token::Restrict,
            TokenPreparse::Bool => Token::Bool,
            TokenPreparse::Complex => Token::Complex,
            TokenPreparse::Imaginary => Token::Imaginary,
            TokenPreparse::Inline => Token::Inline,
            TokenPreparse::Noreturn => Token::Noreturn,
            TokenPreparse::ThreadLocal => Token::ThreadLocal,
            TokenPreparse::Goto => Token::Goto,
            TokenPreparse::Generic => Token::Generic,
            TokenPreparse::StaticAssert => Token::StaticAssert,
            TokenPreparse::Alignas => Token::Alignas,
            TokenPreparse::Alignof => Token::Alignof,
            TokenPreparse::Atomic => Token::Atomic,
            TokenPreparse::Nullptr => Token::Nullptr,
            TokenPreparse::ConstExpr => Token::ConstExpr,
            TokenPreparse::True => Token::True,
            TokenPreparse::False => Token::False,
            TokenPreparse::Typeof => Token::Typeof,
            TokenPreparse::TypeofUnqual => Token::TypeofUnqual,
            TokenPreparse::Private => Token::Private,
            TokenPreparse::Class => Token::Class,
            TokenPreparse::Word(word) => Token::Word(word.to_string()),
            TokenPreparse::Number(number) => Token::Number(number.to_string()),
            _ => return Err(LexerError::UnrecognizedToken),
        })
    }



    Ok(tokens)
}

fn in_string<'input>(lexer: &'input mut Lexer<'input, TokenPreparse<'input>>) -> Result<Token,LexerError> {
    let mut string = String::new();

    while let Some(Ok(token)) = lexer.next() {
        match token {
            TokenPreparse::StringDelimiter => return Ok(Token::String(string)),
            TokenPreparse::CarriageReturnNewline => string.push_str("\r\n"),
            TokenPreparse::Newline => string.push('\n'),
            TokenPreparse::Space => string.push(' '),
            TokenPreparse::Tab => string.push('\t'),
            TokenPreparse::Backslash => string.push('\\'),
            TokenPreparse::Hash => string.push('#'),
            TokenPreparse::Word(word) => string.push_str(&word),
            TokenPreparse::Number(number) => string.push_str(&number),
            TokenPreparse::Character(character) => string.push_str(&character),
            TokenPreparse::Comment(comment) => string.push_str(&comment),
            TokenPreparse::SemiColon => string.push(';'),
            TokenPreparse::Colon => string.push(':'),
            TokenPreparse::Comma => string.push(','),
            TokenPreparse::Period => string.push('.'),
            TokenPreparse::LeftParen => string.push('('),
            TokenPreparse::RightParen => string.push(')'),
            TokenPreparse::LeftBrace => string.push('{'),
            TokenPreparse::RightBrace => string.push('}'),
            TokenPreparse::LeftBracket => string.push('['),
            TokenPreparse::RightBracket => string.push(']'),
            TokenPreparse::Plus => string.push('+'),
            TokenPreparse::Minus => string.push('-'),
            TokenPreparse::Star => string.push('*'),
            TokenPreparse::Divide => string.push('/'),
            TokenPreparse::Modulo => string.push('%'),
            TokenPreparse::Equals => string.push('='),
            TokenPreparse::NotEquals => string.push('!'),
            TokenPreparse::LessThan => string.push('<'),
            TokenPreparse::LessThanOrEqual => string.push_str("<="),
            TokenPreparse::GreaterThan => string.push('>'),
            TokenPreparse::GreaterThanOrEqual => string.push_str(">="),
            TokenPreparse::LogicalAnd => string.push_str("&&"),
            TokenPreparse::LogicalOr => string.push_str("||"),
            TokenPreparse::LogicalNot => string.push('!'),
            TokenPreparse::BitwiseAnd => string.push('&'),
            TokenPreparse::BitwiseOr => string.push('|'),
            TokenPreparse::BitwiseNot => string.push('~'),
            TokenPreparse::BitwiseXor => string.push('^'),
            TokenPreparse::BitwiseLeftShift => string.push_str("<<"),
            TokenPreparse::BitwiseRightShift => string.push_str(">>"),
            TokenPreparse::Assignment => string.push('='),
            TokenPreparse::PlusEquals => string.push_str("+="),
            TokenPreparse::MinusEquals => string.push_str("-="),
            TokenPreparse::StarEquals => string.push_str("*="),
            TokenPreparse::DivideEquals => string.push_str("/="),
            TokenPreparse::ModuloEquals => string.push_str("%="),
            TokenPreparse::BitwiseAndEquals => string.push_str("&="),
            TokenPreparse::BitwiseOrEquals => string.push_str("|="),
            TokenPreparse::BitwiseXorEquals => string.push_str("^="),
            TokenPreparse::BitwiseLeftShiftEquals => string.push_str("<<="),
            TokenPreparse::BitwiseRightShiftEquals => string.push_str(">>="),
            TokenPreparse::Increment => string.push_str("++"),
            TokenPreparse::Decrement => string.push_str("--"),
            TokenPreparse::Arrow => string.push_str("->"),
            TokenPreparse::QuestionMark => string.push('?'),
            TokenPreparse::Auto => string.push_str("auto"),
            TokenPreparse::Double => string.push_str("double"),
            TokenPreparse::Int => string.push_str("int"),
            TokenPreparse::Struct => string.push_str("struct"),
            TokenPreparse::Break => string.push_str("break"),
            TokenPreparse::Else => string.push_str("else"),
            TokenPreparse::Long => string.push_str("long"),
            TokenPreparse::Switch => string.push_str("switch"),
            TokenPreparse::Case => string.push_str("case"),
            TokenPreparse::Enum => string.push_str("enum"),
            TokenPreparse::Register => string.push_str("register"),
            TokenPreparse::Typedef => string.push_str("typedef"),
            TokenPreparse::Char => string.push_str("char"),
            TokenPreparse::Extern => string.push_str("extern"),
            TokenPreparse::Return => string.push_str("return"),
            TokenPreparse::Union => string.push_str("union"),
            TokenPreparse::Const => string.push_str("const"),
            TokenPreparse::Float => string.push_str("float"),
            TokenPreparse::Short => string.push_str("short"),
            TokenPreparse::Unsigned => string.push_str("unsigned"),
            TokenPreparse::Continue => string.push_str("continue"),
            TokenPreparse::For => string.push_str("for"),
            TokenPreparse::Signed => string.push_str("signed"),
            TokenPreparse::Void => string.push_str("void"),
            TokenPreparse::Default => string.push_str("default"),
            TokenPreparse::Goto => string.push_str("goto"),
            TokenPreparse::Sizeof => string.push_str("sizeof"),
            TokenPreparse::Volatile => string.push_str("volatile"),
            TokenPreparse::Do => string.push_str("do"),
            TokenPreparse::If => string.push_str("if"),
            TokenPreparse::Static => string.push_str("static"),
            TokenPreparse::While => string.push_str("while"),
            TokenPreparse::Continue => string.push_str("continue"),
            TokenPreparse::Restrict => string.push_str("restrict"),
            TokenPreparse::Bool => string.push_str("bool"),
            TokenPreparse::Complex => string.push_str("complex"),
            TokenPreparse::Imaginary => string.push_str("imaginary"),
            TokenPreparse::Inline => string.push_str("inline"),
            TokenPreparse::Noreturn => string.push_str("noreturn"),
            TokenPreparse::ThreadLocal => string.push_str("thread_local"),
            TokenPreparse::StaticAssert => string.push_str("static_assert"),
            TokenPreparse::Alignas => string.push_str("alignas"),
            TokenPreparse::Alignof => string.push_str("alignof"),
            TokenPreparse::Atomic => string.push_str("atomic"),
            TokenPreparse::Generic => string.push_str("_Generic"),
            TokenPreparse::ConstExpr => string.push_str("constexpr"),
            TokenPreparse::Nullptr => string.push_str("nullptr"),
            TokenPreparse::Typeof => string.push_str("typeof"),
            TokenPreparse::TypeofUnqual => string.push_str("typeofunqual"),
            TokenPreparse::Private => string.push_str("private"),
            TokenPreparse::Class => string.push_str("class"),
            _ => return Err(LexerError::UnrecognizedToken),
        }
    }

    Err(LexerError::UnterminatedString)
} 


enum PreprocessorState {
    None,
    InDefine(bool),
    InInclude,
    InIf(bool),
    InError,
    InPragma,
    InUndef,
    HashOperator,
}


fn in_preprocessor<'input>(lexer: &'input mut Lexer<'input, TokenPreparse<'input>>) -> Result<Token,LexerError> {
    let mut string = "#".to_string();

    let mut state = PreprocessorState::None;

    while let Some(Ok(token)) = lexer.next() {
        
        match &state {
            PreprocessorState::InDefine(_) => {
                state = PreprocessorState::InDefine(false);
            },
            _ => {},
        }


        match token {
            TokenPreparse::StringDelimiter => string.push('"'),
            TokenPreparse::Newline | TokenPreparse::CarriageReturnNewline => {
                string.push('\n');

                match state {
                    PreprocessorState::InDefine(False) => {
                        return Ok(Token::Macro(string));
                    },
                    PreprocessorState::InInclude => {
                        return Ok(Token::Include(string));
                    },
                    PreprocessorState::InError => {
                        return Ok(Token::Preprocessor(string));
                    },
                    PreprocessorState::InPragma => {
                        return Ok(Token::Preprocessor(string));
                    },
                    PreprocessorState::InUndef => {
                        return Ok(Token::Preprocessor(string));
                    },
                    _ => {},

                }

            },
            TokenPreparse::Space => string.push(' '),
            TokenPreparse::Tab => string.push('\t'),
            TokenPreparse::Backslash => {
                string.push('\\');
                match &state {
                    PreprocessorState::InDefine(_) => {
                        state = PreprocessorState::InDefine(true);
                    },
                    _ => {},
                }
            },
            TokenPreparse::Hash => {
                string.push('#');
                


            },
            TokenPreparse::Word(word) => {
                match &state {
                    PreprocessorState::None => {
                        match word {
                            "define" => {
                                state = PreprocessorState::InDefine(false);
                            },
                            "include" => {
                                state = PreprocessorState::InInclude;
                            },
                            "if" => {
                                state = PreprocessorState::InIf(false);
                            },
                            "ifdef" => {
                                state = PreprocessorState::InIf(false);
                            },
                            "ifndef" => {
                                state = PreprocessorState::InIf(false);
                            },
                            "error" => {
                                state = PreprocessorState::InError;
                            },
                            "pragma" => {
                                state = PreprocessorState::InPragma;
                            },
                            "undef" => {
                                state = PreprocessorState::InUndef;
                            },
                            _ => {
                                state = PreprocessorState::HashOperator;
                            },
                        }
                    },
                    PreprocessorState::InIf(false) => {
                        match word {
                            "endif" => {
                                state = PreprocessorState::InIf(true);
                            }
                            _ => {},
                        }
                    },
                    _ => {},
                }

                string.push_str(&word);
            },
            TokenPreparse::Number(number) => string.push_str(&number),
            TokenPreparse::Character(character) => string.push_str(&character),
            TokenPreparse::Comment(comment) => string.push_str(&comment),
            TokenPreparse::SemiColon => string.push(';'),
            TokenPreparse::Colon => string.push(':'),
            TokenPreparse::Comma => string.push(','),
            TokenPreparse::Period => string.push('.'),
            TokenPreparse::LeftParen => string.push('('),
            TokenPreparse::RightParen => string.push(')'),
            TokenPreparse::LeftBrace => string.push('{'),
            TokenPreparse::RightBrace => string.push('}'),
            TokenPreparse::LeftBracket => string.push('['),
            TokenPreparse::RightBracket => string.push(']'),
            TokenPreparse::Plus => string.push('+'),
            TokenPreparse::Minus => string.push('-'),
            TokenPreparse::Star => string.push('*'),
            TokenPreparse::Divide => string.push('/'),
            TokenPreparse::Modulo => string.push('%'),
            TokenPreparse::Equals => string.push('='),
            TokenPreparse::NotEquals => string.push('!'),
            TokenPreparse::LessThan => string.push('<'),
            TokenPreparse::LessThanOrEqual => string.push_str("<="),
            TokenPreparse::GreaterThan => string.push('>'),
            TokenPreparse::GreaterThanOrEqual => string.push_str(">="),
            TokenPreparse::LogicalAnd => string.push_str("&&"),
            TokenPreparse::LogicalOr => string.push_str("||"),
            TokenPreparse::LogicalNot => string.push('!'),
            TokenPreparse::BitwiseAnd => string.push('&'),
            TokenPreparse::BitwiseOr => string.push('|'),
            TokenPreparse::BitwiseNot => string.push('~'),
            TokenPreparse::BitwiseXor => string.push('^'),
            TokenPreparse::BitwiseLeftShift => string.push_str("<<"),
            TokenPreparse::BitwiseRightShift => string.push_str(">>"),
            TokenPreparse::Assignment => string.push('='),
            TokenPreparse::PlusEquals => string.push_str("+="),
            TokenPreparse::MinusEquals => string.push_str("-="),
            TokenPreparse::StarEquals => string.push_str("*="),
            TokenPreparse::DivideEquals => string.push_str("/="),
            TokenPreparse::ModuloEquals => string.push_str("%="),
            TokenPreparse::BitwiseAndEquals => string.push_str("&="),
            TokenPreparse::BitwiseOrEquals => string.push_str("|="),
            TokenPreparse::BitwiseXorEquals => string.push_str("^="),
            TokenPreparse::BitwiseLeftShiftEquals => string.push_str("<<="),
            TokenPreparse::BitwiseRightShiftEquals => string.push_str(">>="),
            TokenPreparse::Increment => string.push_str("++"),
            TokenPreparse::Decrement => string.push_str("--"),
            TokenPreparse::Arrow => string.push_str("->"),
            TokenPreparse::QuestionMark => string.push('?'),
            TokenPreparse::Auto => string.push_str("auto"),
            TokenPreparse::Double => string.push_str("double"),
            TokenPreparse::Int => string.push_str("int"),
            TokenPreparse::Struct => string.push_str("struct"),
            TokenPreparse::Break => string.push_str("break"),
            TokenPreparse::Else => string.push_str("else"),
            TokenPreparse::Long => string.push_str("long"),
            TokenPreparse::Switch => string.push_str("switch"),
            TokenPreparse::Case => string.push_str("case"),
            TokenPreparse::Enum => string.push_str("enum"),
            TokenPreparse::Register => string.push_str("register"),
            TokenPreparse::Typedef => string.push_str("typedef"),
            TokenPreparse::Char => string.push_str("char"),
            TokenPreparse::Extern => string.push_str("extern"),
            TokenPreparse::Return => string.push_str("return"),
            TokenPreparse::Union => string.push_str("union"),
            TokenPreparse::Const => string.push_str("const"),
            TokenPreparse::Float => string.push_str("float"),
            TokenPreparse::Short => string.push_str("short"),
            TokenPreparse::Unsigned => string.push_str("unsigned"),
            TokenPreparse::Continue => string.push_str("continue"),
            TokenPreparse::For => string.push_str("for"),
            TokenPreparse::Signed => string.push_str("signed"),
            TokenPreparse::Void => string.push_str("void"),
            TokenPreparse::Default => string.push_str("default"),
            TokenPreparse::Goto => string.push_str("goto"),
            TokenPreparse::Sizeof => string.push_str("sizeof"),
            TokenPreparse::Volatile => string.push_str("volatile"),
            TokenPreparse::Do => string.push_str("do"),
            TokenPreparse::If => string.push_str("if"),
            TokenPreparse::Static => string.push_str("static"),
            TokenPreparse::While => string.push_str("while"),
            TokenPreparse::Continue => string.push_str("continue"),
            TokenPreparse::Restrict => string.push_str("restrict"),
            TokenPreparse::Bool => string.push_str("bool"),
            TokenPreparse::Complex => string.push_str("complex"),
            TokenPreparse::Imaginary => string.push_str("imaginary"),
            TokenPreparse::Inline => string.push_str("inline"),
            TokenPreparse::Noreturn => string.push_str("noreturn"),
            TokenPreparse::ThreadLocal => string.push_str("thread_local"),
            TokenPreparse::StaticAssert => string.push_str("static_assert"),
            TokenPreparse::Alignas => string.push_str("alignas"),
            TokenPreparse::Alignof => string.push_str("alignof"),
            TokenPreparse::Atomic => string.push_str("atomic"),
            TokenPreparse::Generic => string.push_str("_Generic"),
            TokenPreparse::ConstExpr => string.push_str("constexpr"),
            TokenPreparse::Nullptr => string.push_str("nullptr"),
            TokenPreparse::Typeof => string.push_str("typeof"),
            TokenPreparse::TypeofUnqual => string.push_str("typeofunqual"),
            TokenPreparse::Private => string.push_str("private"),
            TokenPreparse::Class => string.push_str("class"),
            _ => return Err(LexerError::UnrecognizedToken),
        }
    }
    
    Ok(Token::Preprocessor(string))
} 


mod lexer_test {
    use super::*;
    use logos::{Logos, Lexer};

    #[test]
    fn test_parser() {
        let input = "int main() { return 0; }";
        let mut lexer = Token::lexer("int main() { return 0; }");

        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
    }


    #[test]
    fn test_parser_int() {
        let input = "int main() { return 0; }";
        let mut lexer = Token::lexer("int");

        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
    }
    
    #[test]
    fn test_parser_hello_world() {
        let input = "#include <stdio.h> int main() { printf(\"Hello, World!\");\n return 0; }";;
        let mut lexer = Token::lexer(input);

        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());

    }

}
