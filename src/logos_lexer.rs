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

    #[regex("[0-9]*[.]?[0-9]*([eE]-[^0][0-9]+[uUlL]?[lL]?[lL]?)?|0[xX][0-9a-fA-F]+[uUlL]?[lL]?[lL]?|[^0][0-9]+[uUlL]?[lL]?[lL]?|0[0-7]+", |lex| lex.slice())]
    Number(&'input str),

    //#[regex("\"[^\"]*\"", |lex| lex.slice())]
    //String(&'input str),
    #[token("\"")]
    StringDelimiter,
    #[token("'")]
    SingleQuote,
    
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
    #[token("##")]
    DoubleHash,
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

#[derive(Debug, Clone, PartialEq)]
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
    Type(String),
    //Auto,
    //Double,
    //Int,
    Struct,
    Break,
    Else,
    //Long,
    Switch,
    Case,
    Enum,
    //Register,
    Typedef,
    //Char,
    //Extern,
    Return,
    Union,
    Continue,
    For,
    //Signed,
    //Void,
    Do,
    If,
    Static,
    While,
    Default,
    Goto,
    Sizeof,
    //Volatile,
    //Const,
    //Float,
    //Short,
    //Unsigned,
    
    //todo check some of these for correctness
    Restrict,// can only appear as * restrict
    Inline,
    Alignas,
    Alignof,
    Atomic,
    //Bool,
    //Complex,
    Generic,
    //Imaginary,
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
    UnterminatedCharacter,
    UnrecognizedToken(String),
    BadType(String),
    Empty,
}


enum ParserState {
    Normal,
    InString(String, bool),
    InPreprocessor(String, PreprocessorState),
    InType(String, LastType),
}

enum LastType {
    Type,//   //bool, void, char, short, int, long, float, double, signed, unsigned
    PrefixMod,//const, volatile, auto, register
    SuffixMod,//complex, imaginary
}


#[derive(Debug, Copy,Clone, PartialEq)]
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


pub fn lex<'input>(input: &'input str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();
    
    let mut lexer = TokenPreparse::lexer(input);
    
    let mut state = ParserState::Normal;

    while let Some(Ok(token)) = lexer.next() {
        match token {
            TokenPreparse::StringDelimiter => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InString(String::new(), false);
                    },
                    ParserState::InString(mut string, false) => {
                        tokens.push(Token::String(string));
                        state = ParserState::Normal;
                        continue;
                    },
                    ParserState::InString(mut string, true) => {
                        string.push('"');
                        state = ParserState::InString(string, false);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push('"');
                        state = ParserState::InPreprocessor(string, preproc_state);
                        continue;
                    },
                    _ => {},
                }

                //in_string(&mut lexer)?
            },
            TokenPreparse::Newline => {
                match state {
                    ParserState::InString(mut string,_) => {
                        string.push('\n');
                        state = ParserState::InString(string, false);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push('\n');
                        
                        match preproc_state {
                            PreprocessorState::InDefine(false) => {
                                tokens.push(Token::Macro(string));
                                state = ParserState::Normal;
                                continue;
                            },
                            PreprocessorState::InIf(true) => {
                                tokens.push(Token::Preprocessor(string));
                                state = ParserState::Normal;
                                continue;
                            },
                            PreprocessorState::InInclude => {
                                tokens.push(Token::Include(string));
                                state = ParserState::Normal;
                                continue;
                            },
                            _ => {
                                tokens.push(Token::Preprocessor(string));
                                state = ParserState::Normal;
                                continue;
                            },
                        }
                    },
                    _ => {},
                }
            },
            TokenPreparse::Space => {
                match state {
                    ParserState::InString(mut string,_) => {
                        string.push(' ');
                        state = ParserState::InString(string, false);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,_) => {
                        string.push(' ');
                        state = ParserState::InPreprocessor(string,PreprocessorState::None);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Tab => {
                match state {
                    ParserState::InString(mut string, _) => {
                        string.push('\t');
                        state = ParserState::InString(string, false);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,_) => {
                        string.push('\t');
                        state = ParserState::InPreprocessor(string,PreprocessorState::None);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Backslash => {
                match state {
                    ParserState::InString(mut string, _) => {
                        string.push('\\');
                        state = ParserState::InString(string,true);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, mut preproc_state) => {
                        match preproc_state {
                            PreprocessorState::InDefine(false) => {
                                string.push('\\');
                                state = ParserState::InPreprocessor(string,PreprocessorState::InDefine(true));
                                continue;
                            },
                            _ => {},
                        }
                        state = ParserState::InPreprocessor(string,preproc_state);
                    },
                    _ => {},
                }
            },
            TokenPreparse::Hash => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InPreprocessor("#".to_string(),PreprocessorState::None);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push('#');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('#');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }

                //in_preprocessor(&mut lexer)?
            },
            TokenPreparse::CarriageReturnNewline => {
                match state {
                    ParserState::InString(mut string,_) => {
                        string.push_str("\r\n");
                        state = ParserState::InString(string, false);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push('\n');
                        
                        match preproc_state {
                            PreprocessorState::InDefine(false) => {
                                tokens.push(Token::Macro(string));
                                state = ParserState::Normal;
                                continue;
                            },
                            PreprocessorState::InIf(true) => {
                                tokens.push(Token::Preprocessor(string));
                                state = ParserState::Normal;
                                continue;
                            },
                            PreprocessorState::InInclude => {
                                tokens.push(Token::Include(string));
                                state = ParserState::Normal;
                                continue;
                            },
                            _ => {
                                tokens.push(Token::Preprocessor(string));
                                state = ParserState::Normal;
                                continue;
                            },
                        }
                    },

                    _ => {},
                }
            },
            TokenPreparse::Character(character) => {
                match state {
                    ParserState::InString(mut string, _) => {
                        string.push_str(character);
                        state = ParserState::InString(string, false);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str(character);
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::SingleQuote => {
                match state {
                    ParserState::InString(mut string, _) => {
                        string.push('\'');
                        state = ParserState::InString(string, false);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push('\'');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    _ => {
                        return Err(LexerError::UnterminatedCharacter);
                    },
                }
            },
            TokenPreparse::Comment(comment) => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Comment(comment.to_string()));
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str(comment);
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str(comment);
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(string, _) => {
                        tokens.push(Token::Type(string.to_string()));
                        tokens.push(Token::Comment(comment.to_string()));
                        state = ParserState::Normal;
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::SemiColon => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::SemiColon);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push(';');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push(';');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(string, _) => {
                        tokens.push(Token::Type(string.to_string()));
                        tokens.push(Token::SemiColon);
                        state = ParserState::Normal;
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Colon => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Colon);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push(':');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push(':');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(string, _) => {
                        tokens.push(Token::Type(string.to_string()));
                        tokens.push(Token::Colon);
                        state = ParserState::Normal;
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Comma => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Comma);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push(',');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push(',');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(string, _) => {
                        tokens.push(Token::Type(string.to_string()));
                        tokens.push(Token::Comma);
                        state = ParserState::Normal;
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Period => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Period);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push('.');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('.');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::LeftParen => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::LeftParen);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push('(');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('(');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(string, _) => {
                        tokens.push(Token::Type(string.to_string()));
                        tokens.push(Token::LeftParen);
                        state = ParserState::Normal;
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::RightParen => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::RightParen);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push(')');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push(')');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(string, _) => {
                        tokens.push(Token::Type(string.to_string()));
                        tokens.push(Token::RightParen);
                        state = ParserState::Normal;
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::LeftBrace => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::LeftBrace);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push('{');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('{');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::RightBrace => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::RightBrace);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push('}');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('}');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::LeftBracket => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::LeftBracket);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push('[');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('[');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(string, _) => {
                        tokens.push(Token::Type(string.to_string()));
                        tokens.push(Token::LeftBracket);
                        state = ParserState::Normal;
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::RightBracket => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::RightBracket);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push(']');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push(']');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Plus => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Plus);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push('+');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('+');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Minus => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Minus);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push('-');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('-');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Star => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Star);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push('*');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('*');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(string, _) => {
                        tokens.push(Token::Type(string.to_string()));
                        tokens.push(Token::Star);
                        state = ParserState::Normal;
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Divide => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Divide);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push('/');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('/');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Modulo => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Modulo);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push('%');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('%');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Equals => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Equals);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("==");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("==");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::NotEquals => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::NotEquals);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push_str("!=");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("!=");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::LessThan => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::LessThan);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push('<');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('<');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::LessThanOrEqual => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::LessThanOrEqual);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("<=");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("<=");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::GreaterThan => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::GreaterThan);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push('>');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('>');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::GreaterThanOrEqual => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::GreaterThanOrEqual);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str(">=");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str(">=");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::LogicalAnd => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::LogicalAnd);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("&&");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("&&");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::LogicalOr => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::LogicalOr);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("||");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("||");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::LogicalNot => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::LogicalNot);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push('!');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('!');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::BitwiseAnd => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::BitwiseAnd);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push('&');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('&');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::BitwiseOr => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::BitwiseOr);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push('|');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('|');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::BitwiseNot => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::BitwiseNot);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push('~');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('~');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::BitwiseXor => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::BitwiseXor);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push('^');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('^');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::BitwiseLeftShift => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::BitwiseLeftShift);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push_str("<<");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("<<");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::BitwiseRightShift => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::BitwiseRightShift);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str(">>");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str(">>");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Assignment => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Assignment);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push('=');
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push('=');
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::PlusEquals => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::PlusEquals);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push_str("+=");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("+=");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::MinusEquals => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::MinusEquals);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("-=");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("-=");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::StarEquals => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::StarEquals);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push_str("*=");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("*=");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::DivideEquals => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::DivideEquals);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("/=");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("/=");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::ModuloEquals => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::ModuloEquals);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("%=");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("%=");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::BitwiseAndEquals => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::BitwiseAndEquals);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("&=");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("&=");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::BitwiseOrEquals => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::BitwiseOrEquals);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push_str("|=");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("|=");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::BitwiseXorEquals => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::BitwiseXorEquals);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("^=");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("^=");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::BitwiseLeftShiftEquals => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::BitwiseLeftShiftEquals);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("<<=");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("<<=");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::BitwiseRightShiftEquals => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::BitwiseRightShiftEquals);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str(">>=");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str(">>=");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Increment => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Increment);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("++");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("++");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Decrement => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Decrement);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push_str("--");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("--");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Arrow => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Arrow);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("->");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("->");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::QuestionMark => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::QuestionMark);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("?");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("?");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Auto => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InType("auto".to_string(),LastType::PrefixMod);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("auto");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("auto");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(mut string, LastType::PrefixMod) => {
                        string.push_str(" auto");
                        state = ParserState::InType(string,LastType::PrefixMod);
                        continue;
                    },
                    ParserState::InType(mut string, _) => {
                        return Err(LexerError::BadType(string));
                    },
                    _ => {},
                }
            },
            TokenPreparse::Double => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InType("double".to_string(),LastType::Type);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("double");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("double");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(mut string, LastType::SuffixMod) => {
                        return Err(LexerError::BadType(string));
                    },
                    ParserState::InType(mut string, _) => {
                        string.push_str(" double");
                        state = ParserState::InType(string,LastType::SuffixMod);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Int => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InType("int".to_string(),LastType::Type);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("int");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("int");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(mut string, LastType::SuffixMod) => {
                        return Err(LexerError::BadType(format!("{} int",string)));
                    },
                    ParserState::InType(mut string, _) => {
                        string.push_str(" int");
                        state = ParserState::InType(string,LastType::SuffixMod);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Struct => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Struct);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("struct");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("struct");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Break => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Break);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("break");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("break");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Else => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Else);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("else");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("else");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Long => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InType("long".to_string(), LastType::Type);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("long");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("long");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(mut string, LastType::SuffixMod) => {
                        return Err(LexerError::BadType(string));
                    },
                    ParserState::InType(mut string, _) => {
                        string.push_str(" long");
                        state = ParserState::InType(string,LastType::SuffixMod);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Switch => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Switch);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("switch");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("switch");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Case => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Case);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("case");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("case");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Enum => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Enum);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("enum");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("enum");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Register => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InType("register".to_string(), LastType::PrefixMod);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("register");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("register");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(mut string, LastType::PrefixMod) => {
                        string.push_str(" register");
                        state = ParserState::InType(string,LastType::PrefixMod);
                        continue;
                    },
                    ParserState::InType(mut string, _) => {
                        return Err(LexerError::BadType(string));
                    },
                    _ => {},
                }
            },
            TokenPreparse::Typedef => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Typedef);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("typedef");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("typedef");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Char => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InType("char".to_string(), LastType::Type);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("char");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("char");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(mut string, LastType::SuffixMod) => {
                        return Err(LexerError::BadType(string));
                    },
                    ParserState::InType(mut string, _) => {
                        string.push_str(" char");
                        state = ParserState::InType(string,LastType::Type);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Extern => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InType("extern".to_string(), LastType::PrefixMod);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("extern");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("extern");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(mut string, LastType::PrefixMod) => {
                        string.push_str(" extern");
                        state = ParserState::InType(string,LastType::PrefixMod);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Return => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Return);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("return");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("return");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Union => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Union);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("union");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("union");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Continue => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Continue);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("continue");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("continue");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::For => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::For);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("for");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("for");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Signed => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InType("signed".to_string(), LastType::Type);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("signed");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("signed");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(mut string,LastType::PrefixMod) => {
                        string.push_str(" signed");
                        state = ParserState::InType(string,LastType::Type);
                        continue;
                    },
                    ParserState::InType(mut string,_) => {
                        return Err(LexerError::BadType(format!("{} signed",string)));
                    },
                    _ => {},
                }
            },
            TokenPreparse::Void => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InType("void".to_string(), LastType::Type);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("void");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("void");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(mut string, LastType::PrefixMod) => {
                        string.push_str(" void");
                        state = ParserState::InType(string,LastType::Type);
                        continue;
                    },
                    ParserState::InType(mut string,_) => {
                        return Err(LexerError::BadType(format!("{} void",string)));
                    },
                    _ => {},
                }
            },
            TokenPreparse::Default => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Default);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("default");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("default");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Goto => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Goto);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push_str("goto");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("goto");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Sizeof => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Sizeof);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("sizeof");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("sizeof");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Volatile => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InType("volatile".to_string(), LastType::PrefixMod);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("volatile");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("volatile");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(mut string, LastType::PrefixMod) => {
                        string.push_str(" volatile");
                        state = ParserState::InType(string,LastType::PrefixMod);
                        continue;
                    },
                    ParserState::InType(mut string,_) => {
                        return Err(LexerError::BadType(format!("{} volatile",string)));
                    },
                    _ => {},
                }
            },
            TokenPreparse::Do => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Do);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push_str("do");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("do");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::If => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::If);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,mut preproc_state) => {
                        string.push_str("if");
                        match preproc_state {
                            PreprocessorState::None => {
                                preproc_state = PreprocessorState::InIf(false);
                            },
                            _ => {},
                        }
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("if");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Short => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InType("short".to_string(), LastType::Type);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("short");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("short");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(mut string, LastType::SuffixMod) => {
                        return Err(LexerError::BadType(format!("{} short",string)));
                    },
                    ParserState::InType(mut string, _) => {
                        string.push_str(" short");
                        state = ParserState::InType(string,LastType::Type);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Unsigned => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InType("unsigned".to_string(), LastType::Type);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("unsigned");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("unsigned");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(mut string, LastType::SuffixMod) => {
                        return Err(LexerError::BadType(format!("{} unsigned",string)));
                    },
                    ParserState::InType(mut string, _) => {
                        string.push_str(" unsigned");
                        state = ParserState::InType(string,LastType::Type);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::While => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::While);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("while");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("while");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Default => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Default);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("default");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("default");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Static => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Static);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("static");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("static");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Const => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InType("const".to_string(), LastType::PrefixMod);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("const");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("const");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(mut string, LastType::PrefixMod) => {
                        string.push_str(" const");
                        state = ParserState::InType(string,LastType::PrefixMod);
                    }
                    ParserState::InType(mut string, _) => {
                        return Err(LexerError::BadType(format!("{} const",string)));
                    },
                    _ => {},
                }
            },
            TokenPreparse::Float => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InType("float".to_string(), LastType::Type);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("float");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("float");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(mut string, LastType::PrefixMod) => {
                        string.push_str(" float");
                        state = ParserState::InType(string,LastType::Type);
                    },
                    ParserState::InType(mut string, _) => {
                        return Err(LexerError::BadType(format!("{} float",string)));
                    },

                    _ => {},
                }
            },
            TokenPreparse::Restrict => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Restrict);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("restrict");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("restrict");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Bool => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InType("bool".to_string(), LastType::Type);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push_str("bool");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("bool");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(mut string, LastType::PrefixMod) => {
                        string.push_str(" bool");
                        state = ParserState::InType(string,LastType::Type);
                    },
                    ParserState::InType(mut string, _) => {
                        return Err(LexerError::BadType(format!("{} bool",string)));
                    },
                    _ => {},
                }
            },
            TokenPreparse::Complex => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InType("complex".to_string(), LastType::SuffixMod);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("complex");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("complex");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(mut string, LastType::Type) => {
                        string.push_str(" complex");
                        state = ParserState::InType(string,LastType::SuffixMod);
                    },
                    ParserState::InType(mut string, _) => {
                        return Err(LexerError::BadType(format!("{} complex",string)));
                    },
                    _ => {},
                }
            },
            TokenPreparse::Imaginary => {
                match state {
                    ParserState::Normal => {
                        state = ParserState::InType("imaginary".to_string(), LastType::SuffixMod);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("imaginary");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("imaginary");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(mut string, LastType::Type) => {
                        string.push_str(" imaginary");
                        state = ParserState::InType(string,LastType::SuffixMod);
                    },
                    ParserState::InType(mut string, _) => {
                        return Err(LexerError::BadType(format!("{} imaginary",string)));
                    },
                    _ => {},
                }
            },
            TokenPreparse::Inline => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Inline);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("inline");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("inline");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Noreturn => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Noreturn);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("_Noreturn");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("_Noreturn");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::ThreadLocal => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::ThreadLocal);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("thread_local");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("thread_local");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Goto => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Goto);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("goto");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("goto");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Generic => {
                match  state {
                    ParserState::Normal => {
                        tokens.push(Token::Generic);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("_Generic");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("_Generic");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::StaticAssert => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::StaticAssert);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("static_assert");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("static_assert");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Alignas => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Alignas);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("alignas");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("alignas");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Alignof => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Alignof);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("alignof");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("alignof");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Atomic => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Atomic);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("atomic");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("atomic");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Nullptr => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Nullptr);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string, preproc_state) => {
                        string.push_str("nullptr");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("nullptr");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::ConstExpr => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::ConstExpr);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("constexpr");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("constexpr");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::True => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::True);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("true");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("true");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::False => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::False);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("false");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("false");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Typeof => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Typeof);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("typeof");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("typeof");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::TypeofUnqual => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::TypeofUnqual);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("typeof_unqual");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("typeof_unqual");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Private => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Private);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("private");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("private");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Class => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Class);
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str("class");
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str("class");
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Word(word) => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Word(word.to_string()));
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,mut preproc_state) => {
                        string.push_str(word);
                        match preproc_state {
                            PreprocessorState::None => {
                                match word {
                                    "define" => {
                                        preproc_state = PreprocessorState::InDefine(false);
                                    },
                                    "include" => {
                                        preproc_state = PreprocessorState::InInclude;
                                    },
                                    "if" => {
                                        preproc_state = PreprocessorState::InIf(false);
                                    },
                                    "ifdef" => {
                                        preproc_state = PreprocessorState::InIf(false);
                                    },
                                    "ifndef" => {
                                        preproc_state = PreprocessorState::InIf(false);
                                    },
                                    "error" => {
                                        preproc_state = PreprocessorState::InError;
                                    },
                                    "pragma" => {
                                        preproc_state = PreprocessorState::InPragma;
                                    },
                                    "undef" => {
                                        preproc_state = PreprocessorState::InUndef;
                                    },
                                    _ => {
                                        preproc_state = PreprocessorState::HashOperator;
                                    }
                                }
                            },
                            PreprocessorState::InIf(false) => {

                                match word {
                                    "endif" => {
                                        preproc_state = PreprocessorState::InIf(true);
                                    },
                                    _ => {},
                                }
                            }
                            _ => {},
                        }
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str(word);
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    ParserState::InType(string, _) => {
                        tokens.push(Token::Type(string.to_string()));
                        tokens.push(Token::Word(word.to_string()));
                        state = ParserState::Normal;
                        continue;
                    },
                    _ => {},
                }
            },
            TokenPreparse::Number(number) => {
                match state {
                    ParserState::Normal => {
                        tokens.push(Token::Number(number.to_string()));
                        continue;
                    },
                    ParserState::InPreprocessor(mut string,preproc_state) => {
                        string.push_str(&number.to_string());
                        state = ParserState::InPreprocessor(string,preproc_state);
                        continue;
                    },
                    ParserState::InString(mut string,_) => {
                        string.push_str(&number.to_string());
                        state = ParserState::InString(string,false);
                        continue;
                    },
                    _ => {},
                }
            },
            _ => return Err(LexerError::UnrecognizedToken(format!("{:?}",token))),
        }




    }

    if tokens.len() == 0 {
        return Err(LexerError::Empty);
    }

    Ok(tokens)
}


mod lexer_test {
    use super::*;
    use logos::{Logos, Lexer};

    #[test]
    fn test_parser() {
        let input = "int main() { return 0; }";
        let mut lexer = TokenPreparse::lexer("int main() { return 0; }");

        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
    }

    #[test]
    fn test_preproc_if() {
        let input = "#if 1\nint main() { return 0; }\n#endif";
        println!("{}", input);
        let mut lexer = TokenPreparse::lexer(input);

        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
    }


    #[test]
    fn test_parser_int() {
        let input = "int main() { return 0; }";
        let mut lexer = TokenPreparse::lexer("int");

        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
    }
    
    #[test]
    fn test_parser_hello_world() {
        let input = "#include <stdio.h>\n int main() {\n printf(\"Hello, World!\");\n return 0;\n }";
        let mut lexer = TokenPreparse::lexer(input);

        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());
        println!("{:?}", lexer.next());

    }

    #[test]
    fn test_full_parser() {
        let input = "#include <stdio.h>\n int main() {\n printf(\"Hello, World!\");\n return 0;\n }";
        
        println!("{:?}", lex(input));
    }

}
