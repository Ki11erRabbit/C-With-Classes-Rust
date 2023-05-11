
use std::arch::x86_64::_XCR_XFEATURE_ENABLED_MASK;

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


    fn function_pointer_dec(&mut self) -> Result<AstNode, String> {

        let mut node = AstNode::None;
        let mut name = None;
        let mut return_type = None;
        let mut return_pointer = 0;
        let mut seen_first_star = false;
        let mut pointer = 0;
        let mut array = None;
        let mut seen_open_paren = false;
        let mut seen_close_paren = false;

        while self.tokens.len() != 0 {
            let token = &self.tokens[self.head];
            match token {
                Token::Type(_) => {
                    return_type = Some(Type::from_token(token.clone())?);
                    self.head += 1;
                },
                Token::LeftParen => {
                    if seen_close_paren {

                        return Ok(AstNode::VariableList(VariableList::FunctionPointer(Variable::FunctionPointer {
                            name: name,
                            return_type: return_type.expect("no return type"),
                            return_pointer: return_pointer,
                            pointer: pointer,
                            array: array,
                            arguments: self.function_arguments()?,
                        })));
                    }
                    else {
                        seen_open_paren = true;
                        self.head += 1;
                    }
                },
                Token::RightParen => {
                    seen_close_paren = true;
                    self.head += 1;
                },
                Token::Star => {
                    if seen_open_paren {
                        return_pointer += 1;
                    }
                    else if seen_first_star {
                        pointer += 1;
                    }
                    else {
                        seen_first_star = true;
                    }
                    self.head += 1;
                },
                Token::Word(val) => {
                    if seen_open_paren && seen_first_star {
                        name = Some(val.clone());
                    }
                    else {
                        return Err("Malformed function pointer declaration".to_string());
                    }
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
        let mut word_seen = false;

        while self.tokens.len() != 0 {
            match &self.tokens[self.head] {
                Token::LeftParen => {
                    self.head += 1;
                    if !word_seen {
                        self.head = 0;
                        node = self.function_pointer_dec()?;
                    }
                    else {
                        node = self.function()?;
                    }
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
                    word_seen = true;
                    self.head += 1;
                },//TODO: add array
                _ => {
                    return Err(format!("Unexpected token: {:?}", self.tokens[self.head]));
                },
            }
        }
        Ok(node)
    }

    fn function_arguments(&mut self) -> Result<Vec<FunctionArgument>,String> {

        let mut arguments = Vec::new();
        let mut var_name = None;
        let mut the_type = None;
        let mut pointer = 0;
        let mut num_periods = 0;
        let mut restrict = false;
        
        while self.tokens.len() != 0 {
            let token = &self.tokens[self.head];
            match token {
                Token::Type(_) => {
                    the_type = Some(Type::from_token(token.clone())?);
                },
                Token::Word(name) => {
                    var_name = Some(name.clone());
                },
                Token::Star => {
                    pointer += 1;
                },
                Token::Period => {
                    num_periods += 1;
                    if num_periods > 3 {
                        return Err("Too many periods in function argument".to_string());
                    }
                },
                Token::Restrict => {
                    if pointer != 0 {
                        return Err("Restrict can only be used with pointers".to_string());
                    }
                    restrict = true;
                },
                Token::Comma => {
                    if num_periods != 0 {
                        return Err("Ellipsis in wrong spot.".to_string());
                    }
                    else if var_name.is_none() {
                        arguments.push(FunctionArgument::Type(the_type.expect("no type"), pointer));
                        the_type = None;
                        pointer = 0;
                    }
                    else {
                        arguments.push(FunctionArgument::Variable(the_type.expect("no type"),Variable::BasicVar {
                            name: var_name.expect("no name"),
                            array: None,
                            value: None,
                            pointer: pointer,
                            restrict: restrict,
                        }));
                        var_name = None;
                        the_type = None;
                        pointer = 0;
                        restrict = false;
                    }

                },
                Token::RightParen => {
                    if num_periods == 3 {
                        arguments.push(FunctionArgument::Ellipsis);
                    }
                    else if num_periods < 3 {
                        return Err("Too few periods in function argument".to_string());
                    }
                    else if var_name.is_none() {
                        arguments.push(FunctionArgument::Type(the_type.expect("no type"), pointer));
                        the_type = None;
                        pointer = 0;
                    }
                    else {
                        arguments.push(FunctionArgument::Variable(the_type.expect("no type"),Variable::BasicVar {
                            name: var_name.expect("no name"),
                            array: None,
                            value: None,
                            pointer: pointer,
                            restrict: restrict,
                        }));
                        var_name = None;
                        the_type = None;
                        pointer = 0;
                        restrict = false;
                    }
                    self.head += 1;
                    return Ok(arguments);
                },
                Token::LeftParen => {
                    self.head -= 1;
                    self.head -= pointer;
                    let temp = self.function_pointer_dec()?;
                    match temp {
                        AstNode::VariableList(func_def) => {
                            match func_def {
                                VariableList::FunctionPointer(func) => {
                                    arguments.push(FunctionArgument::FunctionPointer(func));
                                },
                                _ => {
                                    return Err("Expected function".to_string());
                                },
                            }
                        },
                        _ => {
                            return Err("Expected function".to_string());
                        },
                    }
                    
                },
                _ => {
                    return Err(format!("Unexpected token: {:?}", token));
                },

            }

        }

        Err("Unexpected end of file".to_string())
    }

    fn function(&mut self) -> Result<AstNode, String> {
        unimplemented!();
    }

    fn expression(&mut self) -> Result<Expression,String> {
        unimplemented!();
    }


    pub fn parse(&mut self) -> Result<Header, String> {
        let mut header_statements = Vec::new();

        while self.tokens.len() != 0 {
            match self.tokens[0] {
                Token::Preprocessor(_) => {
                    header_statements.append(self.preprocessors()?.iter().map(|x| HeaderStatement::Preprocessor(x.clone())).collect::<Vec<HeaderStatement>>().as_mut());

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
                    let node = self.variable_list_or_function()?;
                    match node {
                        AstNode::VariableList(variable_list) => {
                            header_statements.push(HeaderStatement::Variable(variable_list));
                        },
                        AstNode::Function(function) => {
                            header_statements.push(HeaderStatement::Function(function));
                        },
                        _ => {
                            return Err(format!("Unexpected node: {:?}", node));
                        },
                    }

                },
                Token::Class => {

                },
                Token::Static | Token::Inline => {

                },
                Token::Newline => {
                    self.head += 1;
                    header_statements.push(HeaderStatement::Whitespace);
                },
                _ => {
                    return Err(format!("Unexpected token: {:?}", self.tokens[0]));

                },
            }
            self.tokens = self.tokens[self.head..].to_vec();
        }



        Ok(Header { statements: header_statements })
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

    #[test]
    fn test_global_var_string() {
        let input = "char *a = \"Hello World\";\n";
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
    fn test_global_var_pointers() {
        let input = "int *a, *b, *c;\n";
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
    fn test_global_var_pointer_mix() {
        let input = "int *a, b, *c;\n";
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
    fn test_global_var_fail() {
        let input = "int a, b, c\n";
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return;
            }
        };

        let mut parser = Parser::new(tokens);
        assert!(parser.variable_list_or_function().is_err(), "Parse global variable when it should have failed");
    }

    #[test]
    fn test_global_function_ptr() {
        let input = "int (*func)(int, int);\n";
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return;
            },
        };

        let mut parser = Parser::new(tokens);
        assert!(parser.variable_list_or_function().is_ok(), "Failed to parse global function pointer");
    }

    #[test]
    fn test_global_function_ptr_var() {
        let input = "int (*func)(int x, int y);\n";
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return
            },
        };

        let mut parser = Parser::new(tokens);
        assert!(parser.variable_list_or_function().is_ok(), "Failed to parse global function pointer");
    }

    #[test]
    fn test_global_nested_function_ptr() {
        let input = "int (*func)(int (*)(int, int), int);\n";
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return
            },
        };

        let mut parser = Parser::new(tokens);
        assert!(parser.variable_list_or_function().is_ok(), "Failed to parse global function pointer");
    }
}
