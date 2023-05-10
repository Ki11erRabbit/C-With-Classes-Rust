
use crate::ast::*;
use crate::logos_lexer::Token;


#[derive(Debug, PartialEq, Copy, Clone)]
enum TypedefState {
    Start,
    Struct,
    Union,
    Enum,
    Variable,
    FunctionPrototype,
    Function,
    Class,
    Type,
    PointerType,
    ArrayType,
    End,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum StructState {
    Start,
    Body,
    End,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum EnumState {
    Start,
    Body,
    End,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum UnionState {
    Start,
    Body,
    End,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum FunctionState {
    Start,
    Prototype,
    Arguments,
    End,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum VariableState1 {
    Start,
    List,
    Single,
    Basic,
    Pointer,
    FunctionPointer,
    Array,
    End,
}
#[derive(Debug, PartialEq, Copy, Clone)]
enum VariableState2 {
    List,
    Single,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ClassState {
    Start,
    Member,
    Method,
    MemberOrMethod,
    End,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum TypeState {
    Start,
    PrefixMod,
    SuffixMod,
    Base,
    Composite,
    Identifier,
    End,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum StatementState {
    Start,
    Preprocessor,
    Variable,
    Expression,
    Return,
    If,
    Else,
    While,
    For,
    DoWhile,
    Switch(SwitchState),
    Break,
    Continue,
    Goto,
    Label,
    CodeBlock,
    End,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum SwitchState {
    Start,
    Case,
    Body,
    End,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ExpressionState {
    Start,
    Identifier,
    Literal,
    Parenthesis,
    Sizeof,
    Alignof,
    Cast,
    Call,
    CompoundLiteral,
    InitializerList,
    End,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum OperatorState {
    Start,
    None,
    Unary,
    Binary,
    Ternary,
    End,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ParserState {
    Start,
    Preprocessor,
    Typedef(TypedefState),
    Struct(StructState),
    Enum(EnumState),
    Union(UnionState),
    FunctionOrVariable,
    Function(FunctionState),
    Variable(VariableState1, VariableState2),
    Class(ClassState),
    Type(TypeState),
    CodeBlock,
    Statement(StatementState),
    Expression(ExpressionState, OperatorState),
    End,
}


#[derive(Debug, PartialEq)]
pub struct Parser<'a> {
    token_buffer: Vec<&'a Token>,
    state: ParserState,
    node_buffer: Vec<AstNode>,
}


impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser {
            token_buffer: Vec::new(),
            state: ParserState::Start,
            node_buffer: Vec::new(),
        }
    }

    pub fn add_token<'input: 'a>(&mut self, token: &'input Token) {
        self.token_buffer.push(token);
    }

    pub fn construct_header(& self) -> Result<Header,String> {

        Err("Not implemented".to_string())
    }

    fn parse(token_buffer: &Vec<&'a Token>, start: usize ,state: ParserState, node_buffer: &mut Vec<AstNode>) -> Result<(bool, usize), String> {
        
        match token_buffer[0] {
            Token::Newline => {
                return Err("Extranious Newline".to_string());
            },
            Token::Preprocessor(value) => {
                match state {
                    ParserState::Start => {
                        node_buffer.push(AstNode::Preprocessor( Preprocessor {value: value.clone()}));
                        return Ok(false, start + 1);
                    },
                    ParserState::CodeBlock => {
                        node_buffer.push(AstNode::Statement(Statement::Preprocessor(Preprocessor {value: value.clone()})));
                        return Ok(false, start + 1);
                    }
                    _ => {
                        return Err(format!("Unexpected preprocessor directive: {}", value));
                    }
                }
            },
            Token::Char | Token::Short | Token::Int | Token::Long | 
                Token::Float | Token::Double | Token::Signed |
                Token::Unsigned | Token::Void | Token::Bool => {
                    match state {
                        ParserState::Start | ParserState::Type(TypeState::Base) => {
                            node_buffer.push(AstNode::BaseType(BaseType::from_token(token_buffer[0])));
                            return Self::parse(token_buffer, start +1, ParserState::Type(TypeState::Base), node_buffer);
                        },

                    }
            },
            Token::Word(identifier) => {
                match state {
                    ParserState::Start => {
                        node_buffer.push(AstNode::CompositeType(CompositeType::Identifier(identifier.clone())));
                        return Self::parse(token_buffer, start +1, ParserState::Type(TypeState::Composite), node_buffer);
                    },
                    ParserState::Type(_) => {

                        let pos = 
                    }

                }
            }
            _ => {
                return Err("Unimplemented".to_string());
            }
        }
    }

    pub fn consume_tokens(&mut self) -> bool {

        match Self::parse(&self.token_buffer, 0, self.state, &mut self.node_buffer) {
            Ok((false,pos)) => {
                self.node_buffer.push(node);
                self.token_buffer = self.token_buffer[pos..].to_vec();
                true
            },
            Ok((true,_)) => {
                println!("Error: Unexpected end of node");
                false
            },
            Err(err) => {
                println!("Error: {}", err);
                false
            }
            
        }
    }
}






mod ast_tests {
    #[cfg(test)]
    use super::*;
    use crate::logos_lexer::lex;

    #[test]
    fn test_preprocessor() {
        let input = "#include <stdio.h>\n";
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return;
            }
        };

        let mut parser = Parser::new();

        parser.add_token(&tokens[0]);

        assert!(parser.consume_tokens(), "Failed to parse preprocessor directive");
    }
}
