
use crate::ast::*;
use crate::logos_lexer::Token;


#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
enum StructState {
    Start,
    Body,
    End,
}

#[derive(Debug, PartialEq)]
enum EnumState {
    Start,
    Body,
    End,
}

#[derive(Debug, PartialEq)]
enum UnionState {
    Start,
    Body,
    End,
}

#[derive(Debug, PartialEq)]
enum FunctionState {
    Start,
    Prototype,
    Arguments,
    End,
}

#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
enum VariableState2 {
    List,
    Single,
}

#[derive(Debug, PartialEq)]
enum ClassState {
    Start,
    Member,
    Method,
    MemberOrMethod,
    End,
}

#[derive(Debug, PartialEq)]
enum TypeState {
    Start,
    PrefixMod,
    SuffixMod,
    Base,
    Composite,
    Identifier,
    End,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
enum SwitchState {
    Start,
    Case,
    Body,
    End,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
enum OperatorState {
    Start,
    None,
    Unary,
    Binary,
    Ternary,
    End,
}

#[derive(Debug, PartialEq)]
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

    pub fn consume_tokens(&mut self) -> bool {
        let mut pos = 0;
        let mut head = 0;
        let mut ast_node = AstNode::None;
        loop {
            let mut broken = false;
            let curr_pos = pos;
            for token in self.token_buffer[curr_pos..].iter() {
                match token {
                    Token::Preprocessor(value) => {
                        
                        match self.state {
                            Preprocessor => {
                                ast_node = AstNode::Preprocessor(Preprocessor{value: value.clone()});
                                broken = true;
                                pos += 1;
                                head += 1;
                                break;
                            },
                            CodeBlock => {
                                ast_node = AstNode::Statement(Statement::Preprocessor(Preprocessor{value: value.clone()}));
                                broken = true;
                                pos += 1;
                                head += 1;
                                break;
                            },
                            _ => {
                                head += 1;
                            }
                        }
                    }
                    Token::
                }
            }
            if !broken {
                break;
            }
        }


       
        match ast_node {
            AstNode::None => {
                return false;
            },
            _ => {
                self.node_buffer.push(ast_node);
                return true;
            }
        }

    }


}
