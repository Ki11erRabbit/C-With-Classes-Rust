
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

    fn preprocessors(&mut self) -> Result<Vec<Preprocessor>, String> {
        let mut preprocessor = Vec::new();

        while self.tokens.len() != 0 {
            match &self.tokens[self.head] {
                Token::Preprocessor(data) => {
                    preprocessor.push(Preprocessor {value: data.clone()});
                    self.head += 1;
                },
                _ => {
                    break;
                },
            }
        }

        Ok(preprocessor)
    }

    fn variable_value(&mut self) -> Result<VariableValue,String> {

        match &self.tokens[self.head+1] {
            Token::String(data) => {
                self.head += 2;
                Ok(VariableValue::String(data.clone()))
            },
            _ => {
                self.head += 1;
                Ok(VariableValue::Expression(self.expression()?))
            },
        }


    }

    fn variable_list(&mut self) -> Result<Vec<Variable>,String> {

        let mut variable_list = Vec::new();
        let mut pointer = 0;
        let mut value = None;
        let mut array = None;
        let mut restrict = false;
        
        while self.tokens.len() != 0 {
            match &self.tokens[self.head] {
                Token::Word(name) => {
                    variable_list.push(Variable::BasicVar {
                        name: name.clone(),
                        pointer: pointer,
                        restrict: restrict,
                        array: array,
                        value: value,
                    });
                    pointer = 0;
                    value = None;
                    array = None;
                    restrict = false;
                    self.head += 1;
                },
                Token::Star => {
                    pointer += 1;
                    self.head += 1;
                },
                Token::Restrict => {
                    if pointer != 0 {
                        return Err("Restrict can only be used with pointers".to_string());
                    }
                    restrict = true;
                    self.head += 1;
                },
                Token::Assignment => {
                    value = Some(self.variable_value()?);
                    self.head += 1;
                },
                Token::SemiColon => {
                    self.head += 1;
                    return Ok(variable_list);
                },
                _ => {
                    return Err(format!("Unexpected token: {:?}", self.tokens[self.head]));
                },
            }
        }
        Err("Unexpected end of file".to_string())
    }

    fn variable_dec(&mut self) -> Result<AstNode, String> {

        let mut node = AstNode::None;
        let mut the_type = None;
        let mut pointer = 0;
        let mut value = None;
        let mut restrict = false;
        let mut variable_list = Vec::new();
        
        
        for i in 0..=self.head {
            let token = &self.tokens[i];
            match token {
                Token::Type(_) => {
                    the_type = Some(Type::from_token(token.clone())?);
                },
                Token::Word(name) => {
                    variable_list.push(Variable::BasicVar {
                        name: name.clone(),
                        pointer: pointer,
                        restrict: restrict,
                        array: None,
                        value: value.clone(),
                    });
                },
                Token::Star => {
                    pointer += 1;
                },
                Token::Restrict => {
                    if pointer != 0 {
                        return Err("Restrict can only be used with pointers".to_string());
                    }
                    restrict = true;
                },
                Token::Assignment => {
                    value = Some(self.variable_value()?);
                },
                Token::Comma => {
                    let mut temp = self.variable_list()?;
                    variable_list.append(&mut temp); 
                },
                Token::SemiColon => {
                    return Ok(AstNode::VariableList(VariableList::BasicVars {type_: the_type.expect("no type"), variables: variable_list}));
                },//TODO: add array
                _ => {
                    return Err(format!("Unexpected token: {:?}", token));
                },
            }
        }
        Ok(node)

    }

    fn variable_list_or_function(&mut self) -> Result<AstNode, String> {
        let mut node = AstNode::None;

        while self.tokens.len() != 0 {
            match &self.tokens[self.head] {
                Token::LeftParen => {
                    self.head += 1;
                    node = self.function()?;
                    break;
                },
                Token::Comma | Token::SemiColon | Token::Assignment => {
                    self.head += 1;
                    node = self.variable_dec()?;
                    break;
                },
                Token::Star | Token::Restrict => {
                    self.head += 1;
                },
                Token::Word(_) => {
                    self.head += 1;
                },//TODO: add array
                _ => {
                    return Err(format!("Unexpected token: {:?}", self.tokens[self.head]));
                },
            }
        }
        Ok(node)
    }

    fn function(&mut self) -> Result<AstNode, String> {
        unimplemented!();
    }

    fn expression(&mut self) -> Result<Expression,String> {
        unimplemented!();
    }


    pub fn parse(&mut self) -> Result<Header, String> {        let mut preprocessor = Vec::new();
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
                    preprocessor.append(&mut self.preprocessors()?);
                    self.tokens = self.tokens[self.head..].to_vec();

                },
                Token::Typedef => {

                },
                Token::Struct => {

                },
                Token::Enum => {

                },
                Token::Union => {

                },
                Token::Type(_) => {//Variable, Function

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
        
        assert!(parser.preprocessors().is_ok(), "Failed to parse preprocessor directive");
        //assert!(parser.consume_tokens(), "Failed to parse preprocessor directive");
    }

    #[test]
    fn test_macros() {
        let input = "#define MAX(a, b) ((a) > (b) ? (a) : (b))\n";
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return;
            }
        };

        let mut parser = Parser::new(tokens);
        
        assert!(parser.preprocessors().is_ok(), "Failed to parse preprocessor directive");
        //assert!(parser.consume_tokens(), "Failed to parse preprocessor directive");
    }

    #[test]
    fn test_global_variable() {
        let input = "int a;\n";
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return;
            }
        };

        let mut parser = Parser::new(tokens);

        assert!(parser.variable_list_or_function().is_ok(), "Failed to parse global variable");
    }

    #[test]
    fn test_global_variables() {
        let input = "int a, b, c;\n";
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return;
            }
        };

        let mut parser = Parser::new(tokens);

        assert!(parser.variable_list_or_function().is_ok(), "Failed to parse global variables");
    }
}
