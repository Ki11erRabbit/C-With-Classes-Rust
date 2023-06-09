

use crate::ast::*;
use crate::logos_lexer::Token;

use std::ops::{Range, RangeInclusive};



#[derive(Debug, PartialEq)]
pub struct Parser {
    tokens: Vec<Token>,
    head: usize,
    node_buffer: Vec<AstNode>,
}

pub trait Merge<R> {
    fn merge_tokens(&mut self, other: R);
}

impl Merge<Range<usize>> for Parser {
    fn merge_tokens(&mut self, range: Range<usize>) {
        let mut merge = self.tokens[range.clone()].iter().fold(String::new(), |mut acc, token| {
            acc.push_str(&token.to_string());
            acc.push_str(" ");
            acc
        });
        merge.pop();

        self.tokens.drain(range.clone());
        self.tokens.insert(range.start, Token::Type(merge));
        println!("Merged tokens: {:?}", self.tokens);
    }
}

impl Merge<RangeInclusive<usize>> for Parser {
    fn merge_tokens(&mut self, range: RangeInclusive<usize>) {
        let mut merge = self.tokens[range.clone()].iter().fold(String::new(), |mut acc, token| {
            acc.push_str(&token.to_string());
            acc.push_str(" ");
            acc
        });
        merge.pop();

        self.tokens.drain(range.clone());
        self.tokens.insert(*range.start(), Token::Type(merge));
        println!("Merged tokens: {:?}", self.tokens);
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            head: 0,
            node_buffer: Vec::new(),
        }
    }


    fn preprocessors(&mut self) -> Result<Vec<Preprocessor>, String> {
        let mut preprocessor = Vec::new();

        while self.head < self.tokens.len() {
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

        match &self.tokens[self.head] {
            Token::String(data) => {
                self.head += 1;
                Ok(VariableValue::String(data.clone()))
            },
            _ => {
                Ok(VariableValue::Expression(self.expression(None)?))
            },
        }


    }

    fn variable_list(&mut self) -> Result<Vec<Variable>,String> {

        let mut variable_list = Vec::new();
        let mut var_name = String::new();
        let mut pointer = 0;
        let mut value = None;
        let mut array = None;
        let mut restrict = false;
        
        while self.head < self.tokens.len() {
            match &self.tokens[self.head] {
                Token::Word(name) => {
                    var_name = name.clone();
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
                    variable_list.push(Variable::BasicVar {
                        name: var_name.clone(),
                        pointer: pointer,
                        restrict: restrict,
                        array: array,
                        value: value,
                    });
                    pointer = 0;
                    value = None;
                    array = None;
                    restrict = false;
                    return Ok(variable_list);
                },
                Token::Comma => {
                    variable_list.push(Variable::BasicVar {
                        name: var_name.clone(),
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
                Token::LeftBracket => {
                    array = Some(self.variable_array()?);
                },
                _ => {
                    return Err(format!("Unexpected token in var list: {:?}", self.tokens[self.head]));
                },
            }
        }
        Err("Unexpected end of file".to_string())
    }

    fn variable_dec(&mut self) -> Result<AstNode, String> {

        let start = self.head;
        let mut struct_enum_union = false;
        let mut node = AstNode::None;
        let mut type_pos = 0;
        let mut var_name = String::new();
        let mut the_type = None;
        let mut pointer = 0;
        let mut value = None;
        let mut restrict = false;
        let mut variable_list = Vec::new();
        let mut array = None;
        let mut generic = false;
        
        
        while self.head < self.tokens.len() {
            let token = self.tokens[self.head].clone();
            match token {
                Token::Type(_) => {
                    type_pos = self.head;
                    self.head += 1;
                    //the_type = Some(Type::from_token(token.clone())?);
                },
                Token::Word(name) => {
                    if struct_enum_union {
                        self.merge_tokens(start..self.head);
                        struct_enum_union = false;
                        self.head = self.head - (self.head - start);
                        
                        the_type = Some(Type::from_token(self.tokens[start].clone())?);
                    }
                    else {
                        if generic {
                            the_type = Some(Type::from_token(self.tokens[type_pos].clone())?);
                        }
                        else {
                            the_type = Some(Type::from_token(self.tokens[type_pos].clone())?);
                        }
                        var_name = name.clone();
                    }
                    self.head += 1;
                },
                Token::Star => {
                    if struct_enum_union {
                        the_type = Some(Type::from_token(self.tokens[start].clone())?);
                    }
                    else {
                        if generic {
                            the_type = Some(Type::from_token(self.tokens[type_pos].clone())?);
                        }
                        else {
                            the_type = Some(Type::from_token(self.tokens[type_pos].clone())?);
                        }
                    }
                    self.head += 1;
                    pointer += 1;
                },
                Token::Restrict => {
                    self.head += 1;
                    if pointer != 0 {
                        return Err("Restrict can only be used with pointers".to_string());
                    }
                    restrict = true;
                },
                Token::Assignment => {
                    self.head += 1;
                    value = Some(self.variable_value()?);
                },
                Token::Comma => {
                    self.head += 1;
                    variable_list.push(Variable::BasicVar {
                        name: var_name.clone(),
                        pointer: pointer,
                        restrict: restrict,
                        array: array,
                        value: value.clone(),
                    });
                    pointer = 0;
                    value = None;
                    restrict = false;
                    array = None;
                    
                    
                    let mut temp = self.variable_list()?;
                    variable_list.append(&mut temp); 
                },
                Token::SemiColon => {
                    self.head += 1;
                    variable_list.push(Variable::BasicVar {
                        name: var_name.clone(),
                        pointer: pointer,
                        restrict: restrict,
                        array: array,
                        value: value.clone(),
                    });
                    pointer = 0;
                    value = None;
                    restrict = false;
                    array = None;
                    return Ok(AstNode::VariableList(VariableList::BasicVars {
                        type_: the_type.expect("no type"),
                        variables: variable_list,
                        generic: generic,
                    }));
                },
                Token::LeftBracket => {
                    array = Some(self.variable_array()?);

                },
                Token::Struct | Token::Enum | Token::Union | Token::Tagged => {
                    self.head += 1;
                    struct_enum_union = true;
                },
                Token::Generic => {
                    generic = true;
                    type_pos = self.head;
                    self.head += 1;
                },
                _ => {
                    return Err(format!("Unexpected token in var dec: {:?}", token));
                },
            }
        }
        Err("Unexpected end of file in var dec".to_string())
    }

    fn variable_array(&mut self) -> Result<Vec<VariableArray>,String> {
        let mut array = Vec::new();
        self.head += 1;
        while self.tokens[self.head] == Token::LeftBracket ||(
            self.tokens[self.head] != Token::SemiColon &&
                self.tokens[self.head] != Token::Comma &&
                self.tokens[self.head] != Token::Assignment) {
            match self.tokens[self.head] {
                Token::RightBracket => {
                    self.head += 1;
                    array.push(VariableArray::NoSize);
                },
                Token::LeftBracket => {
                    self.head += 1;
                },
                _ => {
                    array.push(VariableArray::Size(self.expression(None)?));
                    self.head += 1;
                },
            }
        }

        Ok(array)
            
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

        while self.head < self.tokens.len() {
            let token = &self.tokens[self.head];
            match token {
                Token::Type(_) => {
                    return_type = Some(Type::from_token(token.clone())?);
                    self.head += 1;
                },
                Token::LeftParen => {
                    self.head += 1;
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
                    }
                },
                Token::RightParen => {
                    seen_close_paren = true;
                    self.head += 1;
                },
                Token::Star => {
                    if seen_open_paren && seen_first_star {
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
                    self.head += 1;
                    if seen_open_paren && seen_first_star {
                        name = Some(val.clone());
                    }
                    else {
                        return Err("Malformed function pointer declaration".to_string());
                    }
                },//TODO: add array
                Token::LeftBracket => {
                    array = Some(self.variable_array()?);
                },
                _ => {
                    return Err(format!("Unexpected token in function pointer: {:?}", token));
                },

            }

        }
        
        Ok(node)
    }
 
    fn variable_list_or_function(&mut self) -> Result<AstNode, String> {
        let mut node = AstNode::None;
        let mut word_seen = false;
        let mut buffer = self.head;
        self.head -= 1;

        while buffer < self.tokens.len() {
            match &self.tokens[buffer] {
                Token::LeftParen => {
                    if !word_seen {
                        node = self.function_pointer_dec()?;
                    }
                    else {
                        node = self.function()?;
                    }
                    break;
                },
                Token::Comma | Token::SemiColon | Token::Assignment => {
                    node = self.variable_dec()?;
                    break;
                },
                Token::Star | Token::Restrict => {
                    buffer += 1;
                },
                Token::Word(_) => {
                    word_seen = true;
                    buffer += 1;
                },//TODO: add array
                Token::LeftBracket => {
                    node = self.variable_dec()?;
                    break;
                },
                Token::Generic => {
                    buffer += 1;
                },
                _ => {
                    return Err(format!("Unexpected token in var list or func: {:?}", self.tokens[buffer]));
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
        
        while self.head < self.tokens.len() {
            let token = &self.tokens[self.head];
            match token {
                Token::Type(_) => {
                    self.head += 1;
                    the_type = Some(Type::from_token(token.clone())?);
                },
                Token::Word(name) => {
                    self.head += 1;
                    var_name = Some(name.clone());
                },
                Token::Star => {
                    self.head += 1;
                    pointer += 1;
                },
                Token::Period => {
                    self.head += 1;
                    num_periods += 1;
                    if num_periods > 3 {
                        return Err("Too many periods in function argument".to_string());
                    }
                },
                Token::Restrict => {
                    self.head += 1;
                    if pointer != 0 {
                        return Err("Restrict can only be used with pointers".to_string());
                    }
                    restrict = true;
                },
                Token::Comma => {
                    self.head += 1;
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
                    self.head += 1;
                    if num_periods == 3 {
                        arguments.push(FunctionArgument::Ellipsis);
                        self.head += 1;
                    }
                    else if num_periods < 3 && num_periods > 0 {
                        return Err("Too few periods in function argument".to_string());
                    }
                    else if var_name.is_none() && the_type.is_some() {
                        arguments.push(FunctionArgument::Type(the_type.expect("no type"), pointer));
                        the_type = None;
                        pointer = 0;
                        self.head += 1;
                    }
                    else if the_type.is_some() && var_name.is_some() {
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
                        self.head += 1;
                    }
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
                    return Err(format!("Unexpected token in function arguments: {:?}", token));
                },

            }

        }

        Err("Unexpected end of file".to_string())
    }

    fn function(&mut self) -> Result<AstNode, String> {
        let mut name = None;
        let mut arguments = None;
        let mut return_type = None;
        let mut return_pointer = 0;
        let mut inline = false;
        let mut static_ = false;
        let mut generic = false;

        while self.head < self.tokens.len() {
            let token = &self.tokens[self.head];
            println!("Function: {:?}", token);
            match token {
                Token::Word(word) => {
                    name = Some(word.clone());
                    self.head += 1;
                },
                Token::LeftParen => {
                    self.head += 1;
                    arguments = Some(self.function_arguments()?);
                },
                Token::LeftBrace => {
                    self.head += 1;
                    let code_block = self.code_block()?;

                    return Ok(AstNode::Function(Function {
                        name: name.expect("no name"),
                        arguments: arguments.expect("no arguments"),
                        return_type: return_type.expect("no return type"),
                        return_pointer: return_pointer,
                        body: code_block,
                        inline: inline,
                        static_: static_,
                        generic: generic,
                    }));
                },
                Token::Star => {
                    self.head += 1;
                    return_pointer += 1;
                },
                Token::Type(_) => {
                    self.head += 1;
                    return_type = Some(Type::from_token(token.clone())?);
                },
                Token::Inline => {
                    self.head += 1;
                    inline = true;
                },
                Token::Static => {
                    self.head += 1;
                    static_ = true;
                },
                Token::SemiColon => {
                    self.head += 1;
                    return Ok(AstNode::FunctionPrototype(FunctionPrototype {
                        name: name.expect("no name"),
                        arguments: arguments.expect("no arguments"),
                        return_type: return_type.expect("no return type"),
                        return_pointer: return_pointer,
                    }));
                },
                Token::Generic => {
                    self.head += 1;
                    generic = true;
                },
                _ => {
                    return Err(format!("Unexpected token in function: {:?}", token));
                },
            }
            //self.head += 1;
        }

        return Err("Unexpected end of file".to_string());
    }

    fn code_block(&mut self) -> Result<CodeBlock, String> {
        let mut statements = Vec::new();

        while self.head < self.tokens.len() {
            let token = &self.tokens[self.head];
            match token {
                Token::RightBrace => {
                    self.head += 1;
                    return Ok(CodeBlock::Code(StatementList {statements}));
                },
                Token::Preprocessor(_) => {
                    self.head += 1;
                    statements.append(self.preprocessors()?.iter().map(|x| Statement::Preprocessor(x.clone())).collect::<Vec<Statement>>().as_mut());
                },
                Token::Comment(value) => {
                    statements.push(Statement::Comment(value.clone()));
                    self.head += 1;
                },
                Token::Struct | Token::Union | Token::Enum | Token::Tagged |Token::Type(_) => {
                    self.head += 1;
                    let node = self.variable_list_or_function()?;
                    match node {
                        AstNode::VariableList(variable_list) => {
                            statements.push(Statement::VariableList(variable_list));
                        },
                        AstNode::Function(_) => {
                            return Err("Functions cannot be declared inside of a code block".to_string());
                        },
                        _ => {
                            return Err("Expected variable list".to_string());
                        },
                    }
                },
                Token::LeftBrace => {
                    let code_block = self.code_block()?;
                    statements.push(Statement::Block(Box::new(code_block)));
                },
                Token::Return | Token::If | Token::Else | Token::While | Token::For |
                Token::Do | Token::Switch | Token::Case | Token::Default | Token::Break |
                Token::Continue | Token::Goto | Token::SemiColon => {
                    let statement = self.statement()?;
                    statements.push(statement);
                },
                _ => {
                    let expression = self.expression(None)?;
                    statements.push(Statement::Expression(expression));
                    match &self.tokens[self.head] {
                        Token::SemiColon => {
                            self.head += 1;
                        },
                        _ => {
                            return Err(format!("Expected semicolon in code block but found {:?} instead", self.tokens[self.head]));
                        },
                    }
                },
            }
        }

        Err("Unexpected end of file in code block".to_string())
        
    }

    fn statement(&mut self) -> Result<Statement,String> {

        let mut requires_semicolon = false;
        let mut expression = None;
        let statement = None;
        
        while self.head < self.tokens.len() {
            let token = &self.tokens[self.head];

            match token {
                Token::Return => {
                    self.head += 1;
                    expression = Some(self.expression(None)?);
                    requires_semicolon = true;
                },
                Token::Break => {
                    self.head += 1;
                    requires_semicolon = true;
                },
                Token::Continue => {
                    self.head += 1;
                    requires_semicolon = true;
                },
                Token::Word(word) => {
                    self.head += 1;
                    match &self.tokens[self.head] {
                        Token::Colon => {
                            self.head += 1;
                            return Ok(Statement::Label(word.clone()));
                        },
                        _ => {
                            self.head -= 1;
                            expression = Some(self.expression(None)?);
                            requires_semicolon = true;
                        },
                    }
                },
                Token::SemiColon => {
                    self.head += 1;
                    if statement.is_some() {
                        return Ok(statement.expect("no statement"));
                    } else {
                        return Ok(Statement::Expression(expression.expect("no expression")));
                    }
                    
                },
                Token::If => {
                    self.head += 1;
                    let statement = self.statement_if()?;
                    return Ok(statement);
                },
                Token::Else => {
                    self.head += 1;
                    let block_or_statement = self.block_or_statement()?;
                    return Ok(Statement::Else(Box::new(block_or_statement)));
                },
                Token::While => {
                    self.head += 1;
                    let statement = self.statement_while()?;
                    return Ok(statement);
                },
                Token::For => {
                    self.head += 1;
                    let statement = self.statement_for()?;
                    return Ok(statement);
                },
                Token::Do => {
                    self.head += 1;
                    let statement = self.statement_do()?;
                    return Ok(statement);
                },
                Token::Switch => {
                    self.head += 1;
                    let statement = self.statement_switch()?;
                    return Ok(statement);
                },
                Token::Goto => {
                    self.head += 1;
                    match &self.tokens[self.head] {
                        Token::Word(word) => {
                            self.head += 1;
                            match self.tokens[self.head] {
                                Token::SemiColon => {
                                    self.head += 1;
                                    return Ok(Statement::Goto(word.clone()));
                                },
                                _ => {
                                    return Err("Expected semicolon in Goto Statement".to_string());
                                },
                            }
                        },
                        _ => {
                            return Err("Expected label".to_string());
                        },
                    }

                },
                _ => {
                    expression = Some(self.expression(None)?);
                    requires_semicolon = true;
                },
                

            }
            

        }

        if requires_semicolon {
            return Err("Expected semicolon in statement".to_string());
        }

        return Ok(Statement::Expression(expression.expect("no expression")));

    }
    fn statement_if(&mut self) -> Result<Statement,String> {
        let expression = self.conditional_expression()?;
        let block_or_statement = self.block_or_statement()?;

        return Ok(Statement::If(expression, Box::new(block_or_statement)));
    }

    fn statement_else(&mut self) -> Result<Statement,String> {
        let block_or_statement = self.block_or_statement()?;

        return Ok(Statement::Else(Box::new(block_or_statement)));
    }

    fn statement_while(&mut self) -> Result<Statement,String> {
        let expression = self.conditional_expression()?;
        let block_or_statement = self.block_or_statement()?;

        return Ok(Statement::While(expression, Box::new(block_or_statement)));
    }

    fn statement_do(&mut self) -> Result<Statement,String> {
        let block_or_statement = self.block_or_statement()?;
        match self.tokens[self.head] {
            Token::While => {
                self.head += 1;
                let expression = self.conditional_expression()?;
                match self.tokens[self.head] {
                    Token::SemiColon => {
                        self.head += 1;
                        return Ok(Statement::DoWhile(expression,Box::new(block_or_statement)));
                    },
                    _ => {
                        return Err("Expected semicolon".to_string());
                    },
                }
            },
            _ => {
                return Err("Expected while".to_string());
            },
        }
    }

    fn statement_for(&mut self) -> Result<Statement,String> {
        match self.tokens[self.head] {
            Token::LeftParen => {
                self.head += 1;
            },
            _ => {
                return Err("Expected left parenthesis".to_string());
            },
        }
        let mut found_first = false;
        let variable_list_or_statement = match self.tokens[self.head] {
            Token::SemiColon => {
                self.head += 1;
                None
            },
            _ => {
                found_first = true;
                Some(Box::new(self.variable_list_or_statement()?))
            },
        };
        if found_first {
            match self.tokens[self.head] {
                Token::SemiColon => {
                    self.head += 1;
                },
                _ => {
                    return Err("Expected semicolon".to_string());
                },
            }
        }

        let mut found_second = false;
        let expression1 = match self.tokens[self.head] {
            Token::SemiColon => {
                self.head += 1;
                None
            },
            _ => {
                found_second = true;
                Some(self.expression(None)?)
            },
        };
        if found_second {
            match self.tokens[self.head] {
                Token::SemiColon => {
                    self.head += 1;
                },
                _ => {
                    return Err("Expected semicolon".to_string());
                },
            }
        }

        let mut found_third = false;
        let expression2 = match self.tokens[self.head] {
            Token::RightParen => {
                self.head += 1;
                None
            },
            _ => {
                found_third = true;
                Some(self.expression(None)?)
            },
        };
        if found_third {
            match self.tokens[self.head] {
                Token::RightParen => {
                    self.head += 1;
                },
                _ => {
                    return Err("Expected right parenthesis".to_string());
                },
            }
        }
        
        let block_or_statement = self.block_or_statement()?;

        return Ok(Statement::For(variable_list_or_statement, expression1, expression2, Box::new(block_or_statement)));
    }

    fn statement_switch(&mut self) -> Result<Statement, String> {
        let expression = self.conditional_expression()?;
        let mut cases = Vec::new();

        while self.head < self.tokens.len() {
            match self.tokens[self.head] {
                Token::Case => {
                    self.head += 1;
                    let expression = self.conditional_expression()?;
                    match self.tokens[self.head] {
                        Token::Colon => {
                            self.head += 1;
                            let block_or_statement = self.block_or_statement()?;
                            cases.push(SwitchCase {default: false, expression: Some(expression), body: Box::new(block_or_statement)});
                        },
                        _ => {
                            return Err("Expected colon".to_string());
                        },
                    }
                },
                Token::Default => {
                    self.head += 1;
                    match self.tokens[self.head] {
                        Token::Colon => {
                            self.head += 1;
                            let block_or_statement = self.block_or_statement()?;
                            cases.push(SwitchCase {default: true, expression: None, body: Box::new(block_or_statement)});
                        },
                        _ => {
                            return Err("Expected colon".to_string());
                        },
                    }
                },
                Token::RightBrace => {
                    self.head += 1;
                    return Ok(Statement::Switch(expression, cases));
                },
                _ => {
                    return Err("Expected case or default".to_string());
                },
            }
        }


        return Ok(Statement::Switch(expression, cases))
    }
    

    fn conditional_expression(&mut self) -> Result<Expression,String> {
        match self.tokens[self.head] {
            Token::LeftParen => {
                self.head += 1;
                let expression = self.expression(None)?;
                match self.tokens[self.head] {
                    Token::RightParen => {
                        self.head += 1;
                        return Ok(expression);
                    },
                    _ => {
                        return Err("Expected right parenthesis".to_string());
                    },
                }
            },
            _ => {
                return Err("Expected left parenthesis".to_string());
            },
        }
    }

    fn block_or_statement(&mut self) -> Result<BlockOrStatement,String> {
        match self.tokens[self.head] {
            Token::LeftBrace => {
                self.head += 1;
                let code_block = self.code_block()?;
                return Ok(BlockOrStatement::Block(code_block));
            },
            _ => {
                let statement = self.statement()?;
                return Ok(BlockOrStatement::Statement(statement));
            },
        }
    }

    fn variable_list_or_statement(&mut self) -> Result<VariableListOrStatement,String> {
        match self.tokens[self.head] {
            Token::Type(_) => {
                let variable_list = self.variable_list_or_function()?;

                match variable_list {
                    AstNode::VariableList(variable_list) => {
                        return Ok(VariableListOrStatement::VariableList(variable_list));
                    },
                    _ => {
                        return Err("Expected variable list".to_string());
                    },
                }
                
            },//todo: add in structs, unions, enums, etc
            _ => {
                let statement = self.statement()?;
                return Ok(VariableListOrStatement::Statement(statement));
            },
        }
    }

    fn expression(&mut self, expression: Option<Expression>) -> Result<Expression,String> {

        let mut full_expression = None;

        match expression {
            Some(expression) => {
                match self.tokens[self.head] {
                    Token::LeftBracket => {
                        let expr = self.expression(None)?;
                        match &self.tokens[self.head] {
                            RightBracket => {
                                self.head += 1;
                                full_expression = Some(self.expression(Some(Expression::Binary(
                                    BinaryOperator::ArrayAccess,
                                    Box::new(expression),
                                    Box::new(expr),)))?);
                            },
                            _ => {
                                return Err("Expected right bracket".to_string());
                            },
                        }
                        
                    },
                    Token::Period => {
                        self.head += 1;
                        let terminal_expr = self.expression(None)?;
                        full_expression = Some(self.expression(Some(Expression::Binary(
                            BinaryOperator::MemberAccess,Box::new(expression),
                            Box::new(terminal_expr))))?);
                    },
                    Token::Arrow => {
                        self.head += 1;
                        let terminal_expr = self.expression(None)?;
                        full_expression = Some(self.expression(Some(Expression::Binary(
                            BinaryOperator::PointerMemberAccess,Box::new(expression),
                            Box::new(terminal_expr))))?);
                    },
                    Token::LeftParen => {
                        self.head += 1;

                        match &self.tokens[self.head] {
                            Token::RightParen => {
                                self.head += 1;
                                full_expression = Some(self.expression(Some(
                                    Expression::CallFunction(expression.get_value().expect("Not identifier"), None)))?);
                            },
                            _ => {
                                let args = self.expression(None)?;
                                match &self.tokens[self.head] {
                                    Token::RightParen => {
                                        self.head += 1;
                                        full_expression = Some(self.expression(Some(
                                            Expression::CallFunction(expression.get_value().expect("Not identifier"), Some(Box::new(args)))))?);
                                    },
                                    _ => {
                                        return Err("Expected right parenthesis".to_string());
                                    },
                                }
                            },
                        }
                    },
                    Token::LeftBrace => {
                        self.head += 1;
                        match expression {
                            Expression::Identifier(ident) => {
                                full_expression = Some(
                                    Expression::TaggedInitializer(ident, Box::new(self.expression(None)?)));
                                match self.tokens[self.head] {
                                    Token::RightBrace => {
                                        self.head += 1;
                                    },
                                    _ => return Err(format!("Expected right brace, got {:?}", self.tokens[self.head])),
                                }
                            }
                            _ => {
                                return Err(format!("Expected identifier, got {:?}", expression));
                            }

                        }

                    }
                    Token::Plus => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::Add,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::Minus => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::Subtract,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::Star => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::Multiply,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::Divide => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::Divide,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::Modulo => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::Modulo,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::BitwiseLeftShift => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::LeftShift,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::BitwiseRightShift => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::RightShift,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::BitwiseAnd => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::BitwiseAnd,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::BitwiseOr => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::BitwiseOr,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::BitwiseXor => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::BitwiseXor,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::Equals => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::Equal,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::NotEquals => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::NotEqual,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::LessThan => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::LessThan,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::LessThanOrEqual => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::LessThanOrEqual,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::GreaterThan => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::GreaterThan,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::GreaterThanOrEqual => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::GreaterThanOrEqual,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::LogicalAnd => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::LogicalAnd,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::LogicalOr => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::LogicalOr,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::Increment => {
                        self.head += 1;
                        full_expression = Some(Expression::Unary(
                            UnaryOperator::PostIncrement,
                            Box::new(expression),));

                    },
                    Token::Decrement => {
                        self.head += 1;
                        full_expression = Some(Expression::Unary(
                            UnaryOperator::PostDecrement,
                            Box::new(expression),));

                    },
                    Token::Assignment => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::Assign,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::PlusEquals => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::AddAssign,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::MinusEquals => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::SubtractAssign,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::StarEquals => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::MultiplyAssign,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::DivideEquals => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::DivideAssign,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::ModuloEquals => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::ModuloAssign,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::BitwiseLeftShiftEquals => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::LeftShiftAssign,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::BitwiseRightShiftEquals => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::RightShiftAssign,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::BitwiseAndEquals => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::BitwiseAndAssign,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::BitwiseXorEquals => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::BitwiseXorAssign,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::BitwiseOrEquals => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::BitwiseOrAssign,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::Comma => {
                        self.head += 1;
                        full_expression = Some(Expression::Binary(
                            BinaryOperator::Comma,
                            Box::new(expression),
                            Box::new(self.expression(None)?),));

                    },
                    Token::QuestionMark => {
                        self.head += 1;
                        let expr = self.expression(None)?;
                        match self.tokens[self.head] {
                            Token::Colon => {
                                self.head += 1;
                                full_expression = Some(Expression::Ternary(
                                    Box::new(expression),
                                    Box::new(expr),
                                    Box::new(self.expression(None)?),));
                            },
                            _ => {
                                return Err("Expected colon".to_string());
                            },
                        }

                    },
                    _ => {
                        full_expression = Some(Expression::Blank);
                    },
                }
                return Ok(full_expression.unwrap());
            },
            None => {
                match self.tokens[self.head].clone() {
                    Token::Word(ident) => {//check for function call or typedef
                        self.head += 1;
                        full_expression = Some(self.expression(Some(Expression::Identifier(ident)))?);
                    },
                    Token::Number(num) => {
                        self.head += 1;
                        full_expression = Some(self.expression(Some(Expression::Literal(Literal::Number(num))))?);
                    },
                    Token::String(string) => {
                        self.head += 1;
                        full_expression = Some(self.expression(Some(Expression::Literal(Literal::String(string))))?);
                    },
                    Token::Character(character) => {
                        self.head += 1;
                        full_expression = Some(self.expression(Some(Expression::Literal(Literal::Char(character))))?);
                    },
                    Token::True => {
                        self.head += 1;
                        full_expression = Some(self.expression(Some(Expression::Literal(Literal::Bool(true))))?);
                    },
                    Token::False => {
                        self.head += 1;
                        full_expression = Some(self.expression(Some(Expression::Literal(Literal::Bool(false))))?);
                    },
                    Token::Period => {
                        self.head += 1;

                        match &self.tokens[self.head] {
                            Token::Word(ident) => {
                                self.head += 1;
                                full_expression = Some(self.expression(Some(Expression::Unary(UnaryOperator::MemberSet, Box::new(Expression::Identifier(ident.clone())))))?);
                            },
                            _ => {
                                return Err("Expected identifier".to_string());
                            },
                        }
                        
                    },
                     Token::LeftParen => {//check for cast, compoundLiteral
                        self.head += 1;
                        match &self.tokens[self.head] {
                            Token::Type(the_type) => {
                                self.head += 1;
                                match self.tokens[self.head] {
                                    Token::RightParen => {
                                        self.head += 1;
                                        let pointer = match self.tokens[self.head] {
                                            Token::Star => {
                                                self.head += 1;
                                                let mut ptr = 1;
                                                while self.tokens[self.head] == Token::Star {
                                                    self.head += 1;
                                                    ptr += 1;
                                                }
                                                ptr
                                            },
                                            _ => {
                                                0
                                            },
                                        };
                                        full_expression = Some(
                                            Expression::Unary(UnaryOperator::Cast(Type::from_token(self.tokens[self.head - 1].clone())?,
                                                                                  pointer),
                                                              Box::new(self.expression(None)?)));
                                    },
                                    _ => {
                                        return Err("Expected right parenthesis".to_string());
                                    },
                                }
                            },
                            _ => {
                                let expression = self.expression(None)?;
                                match self.tokens[self.head] {
                                    Token::RightParen => {
                                        self.head += 1;
                                        full_expression = Some(Expression::Parentheses(Box::new(expression)));
                                    },
                                    _ => {
                                        return Err("Expected right parenthesis".to_string());
                                    },
                                }
                            },

                        }
                    },
                    Token::Increment => {
                        self.head += 1;
                        full_expression = Some(Expression::Unary(UnaryOperator::PreIncrement, Box::new(self.expression(None)?)));
                    },
                    Token::Decrement => {
                        self.head += 1;
                        full_expression = Some(Expression::Unary(UnaryOperator::PreDecrement, Box::new(self.expression(None)?)));
                    },
                    Token::Star => {
                        self.head += 1;
                        full_expression = Some(Expression::Unary(UnaryOperator::Dereference, Box::new(self.expression(None)?)));
                    },
                    Token::BitwiseAnd => {
                        self.head += 1;
                        full_expression = Some(Expression::Unary(UnaryOperator::AddressOf, Box::new(self.expression(None)?)));
                    },
                    Token::Plus => {
                        self.head += 1;
                        full_expression = Some(Expression::Unary(UnaryOperator::Plus, Box::new(self.expression(None)?)));
                    },
                    Token::Minus => {
                        self.head += 1;
                        full_expression = Some(Expression::Unary(UnaryOperator::Minus, Box::new(self.expression(None)?)));
                    },
                    Token::LogicalNot => {
                        self.head += 1;
                        full_expression = Some(Expression::Unary(UnaryOperator::LogicalNot, Box::new(self.expression(None)?)));
                    },
                    Token::BitwiseNot => {
                        self.head += 1;
                        full_expression = Some(Expression::Unary(UnaryOperator::BitwiseNot, Box::new(self.expression(None)?)));
                    },
                    Token::Sizeof => {
                        self.head += 1;
                        let type_or_expression = self.type_or_expression()?;

                        full_expression = Some(Expression::Sizeof(type_or_expression));

                    },
                    Token::LeftBrace => {
                        self.head += 1;

                        let expr = self.expression(None)?;

                        match self.tokens[self.head] {
                            Token::RightBrace => {
                                self.head += 1;
                                full_expression = Some(Expression::InitializerList(Box::new(expr)));
                            },
                            _ => {
                                return Err("Expected right brace".to_string());
                            },
                        }
                    },
                    _ => {
                        return Err(format!("Unexpected token in expression: {:?}", self.tokens[self.head]));
                    },
                    
                }

                return Ok(full_expression.unwrap());
            },
        }
    }

    fn type_or_expression(&mut self) -> Result<TypeOrExpression, String> {
        match self.tokens[self.head] {
            Token::LeftParen => {
                self.head += 1;
                match self.tokens[self.head] {
                    Token::Type(_) => {
                        let type_ = Type::from_token(self.tokens[self.head].clone())?;
                        self.head += 1;
                        let mut pointer = 0;
                        while self.tokens[self.head] == Token::Star {
                            self.head += 1;
                            pointer += 1;
                        }

                        match self.tokens[self.head] {
                            Token::RightParen => {
                                self.head += 1;
                                return Ok(TypeOrExpression::Type(type_, pointer));
                            },
                            _ => {
                                return Err("Expected right parenthesis".to_string());
                            },
                        }

                    },
                    _ => {
                        return Err("Expected type".to_string());
                    },
                
            }
            },
            _ => {
                return Ok(TypeOrExpression::Expression(Box::new(self.expression(None)?)));
            },
            
        }
    }

    fn struct_dec(&mut self, name: &str) -> Result<AstNode, String> {
        let mut members = Vec::new();
        while self.tokens[self.head] != Token::RightBrace {
            self.head += 1;
            match self.variable_list_or_function()? {
                AstNode::VariableList(variable_list) => {
                    members.push(variable_list);
                },
                AstNode::Function(function) => {
                    return Err("Functions not allowed in structs".to_string());
                },
                _ => {
                    return Err("Expected variable list or function".to_string());
                },
            }
        }

        match self.tokens[self.head] {
            Token::RightBrace => {
                self.head += 1;
                match self.tokens[self.head] {
                    Token::SemiColon => {
                        self.head += 1;
                    },
                    _ => {
                        return Err("Expected semicolon in Struct".to_string());
                    },
                }
                return Ok(AstNode::Struct(Struct {name: name.to_string(), members}));
            },
            _ => {
                return Err("Expected right brace".to_string());
            },
        }
        
    }

    fn union_dec(&mut self, name: &str) -> Result<AstNode, String> {
        let mut members = Vec::new();
        while self.tokens[self.head] != Token::RightBrace {
            self.head += 1;
            match self.variable_list_or_function()? {
                AstNode::VariableList(variable_list) => {
                    members.push(variable_list);
                },
                AstNode::Function(function) => {
                    return Err("Functions not allowed in unions".to_string());
                },
                _ => {
                    return Err("Expected variable list or function".to_string());
                },
            }
        }

        match self.tokens[self.head] {
            Token::RightBrace => {
                self.head += 1;
                match self.tokens[self.head] {
                    Token::SemiColon => {
                        self.head += 1;
                    },
                    _ => {
                        return Err("Expected semicolon in Union".to_string());
                    },
                }
                return Ok(AstNode::Union(Union {name: name.to_string(), members}));
            },
            _ => {
                return Err("Expected right brace".to_string());
            },
        }
    }

    fn enum_dec(&mut self, name: &str) -> Result<AstNode, String> {
        let mut members = Vec::new();
        while self.tokens[self.head] != Token::RightBrace {
            members.push(self.enum_member()?);
            match self.tokens[self.head] {
                Token::Comma => {
                    self.head += 1;
                },
                Token::RightBrace => {
                    break;
                },
                _ => {
                    return Err("Expected comma or right brace".to_string());
                },
            }
        }

        match self.tokens[self.head] {
            Token::RightBrace => {
                self.head += 1;
                match self.tokens[self.head] {
                    Token::SemiColon => {
                        self.head += 1;
                    },
                    _ => {
                        return Err("Expected semicolon in Enum".to_string());
                    },
                }
                return Ok(AstNode::Enum(Enum {name: name.to_string(), members}));
            },
            _ => {
                return Err("Expected right brace".to_string());
            },
        }
    }

    fn enum_member(&mut self) -> Result<EnumMember,String> {
        let mut name = None;
        let mut value = None;

        match &self.tokens[self.head] {
            Token::Word(val) => {
                name = Some(val.clone());
                self.head += 1;
                match &self.tokens[self.head] {
                    Token::Assignment => {
                        self.head += 1;
                        value = Some(self.expression(None)?);
                    },
                    _ => {
                    },
                }
            },

            _ => {
                return Err("Expected identifier".to_string());
            },
        }

        return Ok(EnumMember {name: name.unwrap(), value});
    }

    fn tagged_union_dec(&mut self, name: &str) -> Result<AstNode, String> {
        let mut members = Vec::new();
        while self.tokens[self.head] != Token::RightBrace {
            members.push(self.tagged_union_member()?);
            match self.tokens[self.head] {
                Token::Comma => {
                    self.head += 1;
                },
                Token::RightBrace => {
                    break;
                },
                _ => {
                    return Err("Expected comma or right brace".to_string());
                },
            }
        }

        match self.tokens[self.head] {
            Token::RightBrace => {
                self.head += 1;
                match self.tokens[self.head] {
                    Token::SemiColon => {
                        self.head += 1;
                    },
                    _ => {
                        return Err("Expected semicolon in Tagged Union".to_string());
                    },
                }
                return Ok(AstNode::TaggedUnion(TaggedUnion {name: name.to_string(), members}));
            },
            _ => {
                return Err("Expected right brace".to_string());
            },
        }
    }

    fn tagged_union_member(&mut self) -> Result<TaggedUnionMember,String> {
        let mut name = None;
        let mut value = None;

        while self.tokens[self.head] != Token::Comma && self.tokens[self.head] != Token::RightBrace {
            match &self.tokens[self.head] {
                Token::Word(val) => {
                    self.head += 1;
                    name = Some(val.clone());
                },
                Token::LeftBrace => {
                    self.head += 1;
                    match &self.tokens[self.head] {
                        Token::Word(_) | Token::Type(_) | Token::Enum |
                        Token::Struct | Token::Union | Token::Tagged => {
                            self.head += 1;
                        },
                        _ => {
                            return Err("Expected identifier".to_string());
                        },
                    }
                    value = Some(Vec::new());
                    while self.tokens[self.head] != Token::RightBrace {
                        value.as_mut().unwrap().push(match self.variable_list_or_function()? {
                            AstNode::VariableList(variable_list) => {
                                variable_list
                            },
                            AstNode::Function(_) => {
                                return Err("Functions not allowed in tagged unions".to_string());
                            },
                            _ => {
                                return Err("Expected variable list or function".to_string());
                            },
                        });
                        match self.tokens[self.head] {
                            Token::SemiColon => {
                                self.head += 1;
                            },
                            Token::RightBrace => {
                                self.head += 1;
                                break;
                            },
                            _ => {
                                self.head += 1;
                            },
                        }
                    }
                },
                _ => {
                    return Err("Expected identifier or left brace".to_string());
                },
            }

        }
        
        return Ok(TaggedUnionMember {name: name.unwrap(), value});
    }

    fn compound_type_dec_or_vlist_or_func(&mut self) -> Result<AstNode, String> {
        enum State {
            None,
            Struct,
            Union,
            Enum,
            Tagged
        }
        let mut node = AstNode::None;
        let mut buffer = self.head - 1;
        let mut word_seen = false;
        let mut state = State::None;
        let mut name = None;
        self.head -= 1;

        while buffer < self.tokens.len() {
            let token = &self.tokens[buffer];
            match token {
                Token::Struct => {
                    buffer += 1;
                    match state {
                        State::None => {
                            state = State::Struct;
                        },
                        _ => {
                            return Err(format!("Unexpected token: {:?}", token));
                        },
                    }
                },
                Token::Enum => {
                    buffer += 1;
                    match state {
                        State::None => {
                            state = State::Enum;
                        },
                        _ => {
                            return Err(format!("Unexpected token: {:?}", token));
                        },
                    }
                },
                Token::Union => {
                    buffer += 1;
                    match state {
                        State::None => {
                            state = State::Union;
                        },
                        _ => {
                            return Err(format!("Unexpected token: {:?}", token));
                        },
                    }
                },
                Token::Tagged => {
                    buffer += 1;
                    match state {
                        State::None => {
                            state = State::Tagged;
                        },
                        _ => {
                            return Err(format!("Unexpected token: {:?}", token));
                        },
                    }
                },
                Token::Word(val) => {
                    buffer += 1;
                    if word_seen {
                        self.merge_tokens(0..=buffer);
                        return self.variable_list_or_function();
                    }
                    else {
                        name = Some(val.clone());
                        word_seen = true;
                    }
                },
                Token::Type(_) => {
                    buffer += 1;
                },
                Token::LeftBrace => {
                    buffer += 1;
                    self.head = buffer;
                    match state {
                        State::Struct => {
                            node = self.struct_dec(&name.expect("Struct has no name"))?;
                        },
                        State::Union => {
                            node = self.union_dec(&name.expect("Union has no name"))?;
                        },
                        State::Enum => {
                            node = self.enum_dec(&name.expect("Enum has no name"))?;
                        },
                        State::Tagged => {
                            node = self.tagged_union_dec(&name.expect("Tagged has no name"))?;
                        },
                        _ => {
                            return Err(format!("Unexpected token: {:?}", token));
                        },
                    }
                    return Ok(node);
                },
                _ => {
                    return Err(format!("Unexpected token: {:?}", token));
                },
                
            }
        }
        Err("Unexpected end of file".to_string())
    }

    pub fn class(&mut self) -> Result<Class, String> {
        let mut abstract_ = false;
        let mut generic = None;
        let mut parent = None;
        let mut name = None;
        let mut members = Vec::new();


        while self.head < self.tokens.len() {
            match &self.tokens[self.head] {
                Token::Class => {
                    self.head += 1;
                },
                Token::Abstract => {
                    abstract_ = true;
                    self.head += 1;
                },
                /*Token::LessThan => {
                    self.head += 1;
                    match &self.tokens[self.head] {
                        Token::Word(val) => {
                            generic = Some(val.clone());
                            self.head += 1;
                        },
                        _ => {
                            return Err("Expected identifier".to_string());
                        },
                    }
                    match &self.tokens[self.head] {
                        Token::GreaterThan => {
                            self.head += 1;
                        },
                        _ => {
                            return Err("Expected >".to_string());
                        },
                    }
                },*/
                Token::Colon => {
                    self.head += 1;
                    match &self.tokens[self.head] {
                        Token::Word(val) => {
                            parent = Some(val.clone());
                            self.head += 1;
                        },
                        _ => {
                            return Err("Expected identifier".to_string());
                        },
                    }
                },
                Token::Word(val) => {
                    name = Some(val.clone());
                    self.head += 1;
                },
                Token::LeftBrace => {
                    self.head += 1;
                    while self.tokens[self.head] != Token::RightBrace {
                        members.push(self.class_member(abstract_)?);
                    }
                    match &self.tokens[self.head] {
                        Token::RightBrace => {
                            self.head += 1;
                            return Ok(Class {abstract_, generic, parent, name: name.unwrap(), members});
                        },
                        _ => {
                            return Err("Expected }".to_string());
                        },
                    }
                },
                _ => {
                    return Err("Expected identifier or left brace".to_string());
                },
            }

        }
        Err("Unexpected end of file in Class".to_string())
    }

    fn class_member(&mut self, abstract_: bool) -> Result<ClassMember,String> {
        let mut word_seen = false;
        let mut buffer = self.head;
        self.head -= 1;

        while buffer < self.tokens.len() {
            match &self.tokens[buffer] {
                Token::LeftParen => {
                    if !word_seen {
                        match self.function_pointer_dec()? {
                            AstNode::VariableList(val) => {
                                return Ok(ClassMember::Variable(val));
                            },
                            _ => {
                                return Err("Expected variable list".to_string());
                            },
                        }
                    }
                    else {
                        match self.function()? {
                            AstNode::Function(val) => {
                                return Ok(ClassMember::Method(Method::Normal((val))));
                            },
                            AstNode::FunctionPrototype(val) => {
                                return Ok(ClassMember::Method(Method::Abstract((val))));
                            },
                            _ => {
                                return Err("Expected function".to_string());
                            },
                        }
                    }
                },
                Token::Comma | Token::SemiColon | Token::Assignment => {
                    match self.variable_dec()? {
                        AstNode::VariableList(val) => {
                            return Ok(ClassMember::Variable(val));
                        },
                        _ => {
                            return Err("Expected variable list".to_string());
                        },
                    }
                },
                Token::Star | Token::Restrict => {
                    buffer += 1;
                },
                Token::Word(_) => {
                    word_seen = true;
                    buffer += 1;
                },
                Token::LeftBracket => {
                    match self.variable_dec()? {
                        AstNode::VariableList(val) => {
                            return Ok(ClassMember::Variable(val));
                        },
                        _ => {
                            return Err("Expected variable list".to_string());
                        },
                    }
                },
                Token::Operator => {
                    return Ok(ClassMember::OperatorOverload(self.operator()?));

                },
                _ => {
                    return Err(format!("Unexpected token in class member: {:?}", self.tokens[buffer]));
                },
            }
        }
        Err("Unexpected end of file in class member".to_string())
    }

    fn operator(&mut self) -> Result<OperatorOverload,String> {
        let mut name = None;
        let mut arguments = None;
        let mut return_type = None;
        let mut return_pointer = 0;

        while self.head < self.tokens.len() {
            let token = &self.tokens[self.head];
            match token {
                Token::LeftParen => {
                    self.head += 1;

                    name = match &self.tokens[self.head] {
                        Token::Plus | Token::Minus | Token::Star | Token::Divide |
                        Token::Modulo | Token::BitwiseAnd | Token::BitwiseOr |
                        Token::BitwiseXor | Token::BitwiseNot | Token::BitwiseLeftShift |
                        Token::BitwiseRightShift | Token::Equals | Token::NotEquals |
                        Token::LessThan | Token::GreaterThan | Token::LessThanOrEqual |
                        Token::GreaterThanOrEqual | Token::LogicalAnd | Token::LogicalOr |
                        Token::LogicalNot | Token::Increment | Token::Decrement =>
                            Some(self.tokens[self.head].to_string()),
                        _ => {
                            return Err("Expected operator".to_string());
                        },
                    };
                    self.head += 1;
                    match &self.tokens[self.head] {
                        Token::RightParen => {
                            self.head += 1;
                        },
                        _ => {
                            return Err("Expected )".to_string());
                        },
                    }
                    match &self.tokens[self.head] {
                        Token::LeftParen => {
                            self.head += 1;
                        },
                        _ => {
                            return Err("Expected (".to_string());
                        },
                    }
                    arguments = Some(self.function_arguments()?);
                },
                Token::LeftBrace => {
                    self.head += 1;
                    let code_block = self.code_block()?;

                    return Ok(OperatorOverload::Normal {
                        op: name.expect("no name"),
                        arguments: arguments.expect("no arguments"),
                        return_type: return_type.expect("no return type"),
                        return_pointer: return_pointer,
                        body: code_block,
                    });
                },
                Token::Star => {
                    self.head += 1;
                    return_pointer += 1;
                },
                Token::Type(_) => {
                    self.head += 1;
                    return_type = Some(Type::from_token(token.clone())?);
                },
                Token::SemiColon => {
                    self.head += 1;
                    return Ok(OperatorOverload::Abstract {
                        op: name.expect("no name"),
                        arguments: arguments.expect("no arguments"),
                        return_type: return_type.expect("no return type"),
                        return_pointer: return_pointer,
                    });
                }
                _ => {
                    return Err(format!("Unexpected token in function: {:?}", token));
                },
            }
            //self.head += 1;
        }

        return Err("Unexpected end of file".to_string());
    }

    pub fn parse(&mut self) -> Result<Header, String> {
        let mut header_statements = Vec::new();

        if self.tokens.len() == 0 {
            return Err("No tokens".to_string());
        }

        while self.tokens.len() != 0 {
            println!("{:?}", self.tokens);
            println!("Parsing token {:?}", self.tokens[0]);
            match self.tokens[0] {
                Token::Preprocessor(_) => {
                    header_statements.append(self.preprocessors()?.iter().map(|x| HeaderStatement::Preprocessor(x.clone())).collect::<Vec<HeaderStatement>>().as_mut());

                },
                Token::Typedef => {
                    self.head += 1;

                },
                Token::Struct | Token::Enum | Token::Union | Token::Tagged => {
                    self.head += 1;
                    let node = self.compound_type_dec_or_vlist_or_func()?;

                    match node {
                        AstNode::Struct(struct_) => {
                            header_statements.push(HeaderStatement::Struct(struct_));
                        },
                        AstNode::VariableList(variable_list) => {
                            header_statements.push(HeaderStatement::Variable(variable_list));
                        },
                        AstNode::Function(function) => {
                            header_statements.push(HeaderStatement::Function(function));
                        },
                        AstNode::Enum(enum_) => {
                            header_statements.push(HeaderStatement::Enum(enum_));
                        },
                        AstNode::Union(union_) => {
                            header_statements.push(HeaderStatement::Union(union_));
                        },
                        AstNode::TaggedUnion(tagged) => {
                            header_statements.push(HeaderStatement::TaggedUnion(tagged));
                        },
                        _ => {
                            return Err(format!("Unexpected node: {:?}", node));
                        },
                    }
                    
                },
                Token::Type(_) => {//Variable, Function
                    self.head += 1;
                    let node = self.variable_list_or_function()?;
                    println!("Node: {:?}", node);
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
                Token::Class | Token::Abstract => {
                    header_statements.push(HeaderStatement::Class(self.class()?));
                },
                Token::Static | Token::Inline => {
                    self.head += 1;
                    let node = self.variable_list_or_function()?;
                    match node {
                        AstNode::Function(function) => {
                            header_statements.push(HeaderStatement::Function(function));
                        },
                        _ => {
                            return Err(format!("Unexpected node: {:?}", node));
                        },
                    }

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
            self.head = 0;
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
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return;
            }
        };

        let mut parser = Parser::new(tokens);
        
        assert!(parser.parse().is_ok(), "Failed to parse preprocessor directive");
        //assert!(parser.consume_tokens(), "Failed to parse preprocessor directive");
    }

    #[test]
    fn test_macros() {
        let input = "#define MAX(a, b) ((a) > (b) ? (a) : (b))\n";
        println!("Input: {}", input);
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
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return;
            }
        };

        let mut parser = Parser::new(tokens);

        assert!(parser.parse().is_ok(), "Failed to parse global variable");
    }

    #[test]
    fn test_global_variables() {
        let input = "int a, b, c;\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return;
            }
        };

        let mut parser = Parser::new(tokens);

        assert!(parser.parse().is_ok(), "Failed to parse global variables");
    }

    #[test]
    fn test_global_var_string() {
        let input = "char *a = \"Hello World\";\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return;
            }
        };
        println!("Tokens: {:?}", tokens);

        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(), "Failed to parse global variable");
    }

    #[test]
    fn test_global_var_pointers() {
        let input = "int *a, *b, *c;\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return;
            }
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        assert!(parser.parse().is_ok(), "Failed to parse global variable");
    }

    #[test]
    fn test_global_var_pointer_mix() {
        let input = "int *a, b, *c;\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return;
            }
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        assert!(parser.parse().is_ok(), "Failed to parse global variable");
    }

    #[test]
    fn test_global_var_fail() {
        let input = "int a, b, c\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return;
            }
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        assert!(parser.parse().is_err(), "Parse global variable when it should have failed");
    }

    #[test]
    fn test_global_function_ptr() {
        let input = "int (*func)(int, int);\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return;
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        assert!(parser.parse().is_ok(), "Failed to parse global function pointer");
    }

    #[test]
    fn test_global_function_ptr_var() {
        let input = "int (*func)(int x, int y);\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        assert!(parser.parse().is_ok(), "Failed to parse global function pointer");
    }

    #[test]
    fn test_global_nested_function_ptr() {
        let input = "int (*func)(int (*)(int, int), int);\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        assert!(parser.parse().is_ok(), "Failed to parse global function pointer");
    }

    #[test]
    fn test_simple_function() {
        let input = "int main() { return 0; }\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(), "Failed to parse simple function");
    }

    #[test]
    fn test_simple_program() {
        let input = "int main() { return 0; }\nint a;\nint b;\nint c;\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                return
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(), "Failed to parse simple program");
    }

    #[test]
    fn test_hello_world_program() {
        let input = "#include <stdio.h>\nint main() { printf(\"Hello World\"); return 0; }\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                return
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(), "Failed to parse hello world program");
}
    #[test]
    fn test_math_program() {
        let input = "int main() { return 1 + 2 * 3; }\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                return
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(), "Failed to parse math program");
    }

    #[test]
    fn test_nested_expressions() {
        let input = "int main() { return 1 + (2 * 3); }\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                return
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(), "Failed to parse nested expressions");
    }

    #[test]
    fn test_variables_in_function() {
        let input = "int main() { int a; int b; int c; return 0; }\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                return
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(), "Failed to parse variables in function");
    }

    #[test]
    fn test_global_var_assignment() {
        let input = "int a = 0;\nint b = 1;\nint c = 2;\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                return
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(), "Failed to parse global variable assignment");
    }

    #[test]
    fn test_global_var_assignment_with_expr() {
        let input = "int a = 0 + 1;\nint b = 1 + 2;\nint c = 2 + 3;\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                return
            },
        };
        
        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(), "Failed to parse global variable assignment with expression");
    }

    #[test]
    fn test_struct_access() {
        let input = "struct foo { int a; int b; int c; };\nint main() { struct foo f; f.a = 0; f.b = 1; f.c = 2; return 0; }\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                return
            },
        };
        
        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(), "Failed to parse struct access");
    }

    #[test]
    fn test_method_call() {
        let input = "int main() {\n clss.method();\n return 0;\n}\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return
            },
        };
        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(), "Failed to parse method call");
    }

    #[test]
    fn test_union_access() {
        let input = "union foo { int a; int b; int c; };\nint main() { union foo f; f.a = 0; f.b = 1; f.c = 2; return 0; }\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                assert!(false);
                println!("Error: {:?}", err);
                return
            },
        };
        
        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(), "Failed to parse union access");
    }

    #[test]
    fn test_enum() {
        let input = "enum foo { a, b, c };\nint main() { enum foo f; f = a; return 0; }\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                assert!(false);
                println!("Error: {:?}", err);
                return
            },
        };
        
        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(), "Failed to parse enum");
    }

    #[test]
    fn test_simple_tagged_union() {
        let input = "tagged foo { a,b,c};\nint main() { tagged foo f; f = a; return 0; }\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                assert!(false);
                println!("Error: {:?}", err);
                return
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(),"Failed to parse simple tagged union");

    }

    #[test]
    fn test_complex_tagged_union() {
        let input = "tagged foo { a {int a, b, c;}, b {char *a; int b;}, c {int a; char *b;}};\nint main() { tagged foo f; f = a {1,2,3}; return 0; }\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                assert!(false);
                println!("Error: {:?}", err);
                return
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(),"Failed to parse complex tagged union");

    }

    #[test]
    fn test_global_array_dec() {
        let input = "int a[10];\nint main() { return 0; }\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                assert!(false);
                println!("Error: {:?}", err);
                return
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(),"Failed to parse global array declaration");
    }

    #[test]
    fn test_global_var_mix_with_array() {
        let input = "int a[10], b;\nint main() { return 0; }\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                assert!(false);
                println!("Error: {:?}", err);
                return
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(),"Failed to parse global array declaration");
    }

    #[test]
    fn test_global_var_string_array_assign() {
        let input = "char *a[10] = {\"Hello\", \"World\"};\nint main() { return 0; }\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                assert!(false);
                println!("Error: {:?}", err);
                return
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(),"Failed to parse global array declaration");
    }

    #[test]
    fn test_global_var_string_array_assign_no_size() {
        let input = "char a[] = \"Hello\";\nint main() { return 0; }\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(),"Failed to parse global array declaration");
    }
    

    #[test]
    fn test_global_array_dec_4d() {
        let input = "int a[10][10][10][10];\nint main() { return 0; }\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                assert!(false);
                println!("Error: {:?}", err);
                return
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(),"Failed to parse global array declaration");
    }

    #[test]
    fn test_array_in_function() {
        let input = "int main() { int a[10]; return 0; }\n";
        println!("Input: {}", input);
        let tokens = match lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("Error: {:?}", err);
                assert!(false);
                return
            },
        };

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("Result: {:?}", result);
        assert!(result.is_ok(),"Failed to parse array in function");
    }

}
