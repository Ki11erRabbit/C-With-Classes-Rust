
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
pub struct Parser {
    tokens: Vec<Token>,
    head: usize,
    state: ParserState,
    node_buffer: Vec<AstNode>,
}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: Vec::new(),
            head: 0,
            state: ParserState::Start,
            node_buffer: Vec::new(),
        }
    }


    pub fn parse(&mut self) -> Result<Header, String> {
        let mut preprocessor = Vec::new();
        let mut typedefs = Vec::new();
        let mut structs = Vec::new();
        let mut enums = Vec::new();
        let mut unions = Vec::new();
        let mut function_prototypes = Vec::new();
        let mut functions = Vec::new();
        let mut variables = Vec::new();
        let mut classes = Vec::new();

        while self.tokens.len() != 0 {
            match self.tokens[0] {
                Token::Preprocessor(_) => {

                },
                Token::Typedef => {

                },
                Token::Struct => {

                },
                Token::Enum => {

                },
                Token::Union => {

                },
                Token::Type(_) => {

                },
                Token::Class => {

                },
                _ => {
                    return Err(format!("Unexpected token: {:?}", self.tokens[0]));

                },



            }
        }



        Ok(Header {
            preprocessor,
            typedefs,
            structs,
            enums,
            unions,
            function_prototypes,
            functions,
            variables,
            classes,
        })
    }
}






#[cfg(test)]
mod ast_tests {
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

        let mut parser = Parser::new(tokens);


        //assert!(parser.consume_tokens(), "Failed to parse preprocessor directive");
    }
}
