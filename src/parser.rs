

use crate::ast::*;
use crate::logos_lexer::Token;




#[derive(Debug, PartialEq)]
pub struct Parser {
    tokens: Vec<Token>,
    head: usize,
    node_buffer: Vec<AstNode>,
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
                self.head += 1;
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
                _ => {
                    return Err(format!("Unexpected token: {:?}", self.tokens[self.head]));
                },
            }
        }
        Err("Unexpected end of file".to_string())
    }

    fn variable_dec(&mut self) -> Result<AstNode, String> {

        let mut node = AstNode::None;
        let mut var_name = String::new();
        let mut the_type = None;
        let mut pointer = 0;
        let mut value = None;
        let mut restrict = false;
        let mut variable_list = Vec::new();
        
        
        while self.head < self.tokens.len() {
            let token = &self.tokens[self.head];
            match token {
                Token::Type(_) => {
                    self.head += 1;
                    the_type = Some(Type::from_token(token.clone())?);
                },
                Token::Word(name) => {
                    var_name = name.clone();
                    self.head += 1;
                },
                Token::Star => {
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
                        array: None,
                        value: value.clone(),
                    });
                    pointer = 0;
                    value = None;
                    restrict = false;
                    
                    
                    let mut temp = self.variable_list()?;
                    variable_list.append(&mut temp); 
                },
                Token::SemiColon => {
                    self.head += 1;
                    variable_list.push(Variable::BasicVar {
                        name: var_name.clone(),
                        pointer: pointer,
                        restrict: restrict,
                        array: None,
                        value: value.clone(),
                    });
                    pointer = 0;
                    value = None;
                    restrict = false;
                    return Ok(AstNode::VariableList(VariableList::BasicVars {type_: the_type.expect("no type"), variables: variable_list}));
                },//TODO: add array
                _ => {
                    return Err(format!("Unexpected token: {:?}", token));
                },
            }
        }
        Err("Unexpected end of file".to_string())
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

        while self.head < self.tokens.len() {
            match &self.tokens[self.head] {
                Token::LeftParen => {
                    self.head = 0;
                    if !word_seen {
                        node = self.function_pointer_dec()?;
                    }
                    else {
                        node = self.function()?;
                    }
                    break;
                },
                Token::Comma | Token::SemiColon | Token::Assignment => {
                    self.head = 0;
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
                    return Err(format!("Unexpected token: {:?}", token));
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
                }
                _ => {
                    return Err(format!("Unexpected token: {:?}", token));
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
                Token::Type(_) => {
                    let node = self.variable_list_or_function()?;
                    match node {
                        AstNode::VariableList(variable_list) => {
                            statements.push(Statement::VariableList(variable_list));
                        },
                        AstNode::Function(_) => {
                            return Err("Functions cannot be declared inside of a code block".to_string());
                        },
                        _ => {
                            return Err("Expected variable list or function".to_string());
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
                },
            }
        }

        Err("Unexpected end of file".to_string())
        
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
                                    return Err("Expected semicolon".to_string());
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
            return Err("Expected semicolon".to_string());
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

                        match self.tokens[self.head].clone() {
                            Token::LeftParen => {
                                self.head += 1;

                                let arguments = match self.expression(None)? {
                                    Expression::Blank => {
                                        None
                                    },
                                    _ => {
                                        Some(Box::new(self.expression(None)?))
                                    },

                                };

                                match self.tokens[self.head] {
                                    Token::RightParen => {
                                        self.head += 1;
                                        full_expression = Some(self.expression(Some(Expression::CallFunction(ident, arguments)))?);
                                    },
                                    _ => {
                                        return Err("Expected right parenthesis".to_string());
                                    },
                                }
                                
                            },
                            Token::Period => {
                                self.head += 1;
                                let member = match self.tokens[self.head].clone() {
                                    Token::Word(ident) => {
                                        self.head += 1;
                                        ident
                                    },
                                    _ => {
                                        return Err("Expected identifier".to_string());
                                    },
                                };

                                match self.tokens[self.head].clone() {
                                    Token::LeftParen => {
                                        self.head += 1;

                                        let arguments = match self.expression(None)? {
                                            Expression::Blank => {
                                                None
                                            },
                                            _ => {
                                                Some(Box::new(self.expression(None)?))
                                            },

                                        };

                                        match self.tokens[self.head] {
                                            Token::RightParen => {
                                                self.head += 1;
                                                full_expression = Some(self.expression(Some(Expression::CallMethod(false, ident, member.clone(), arguments)))?);
                                            },
                                            _ => {
                                                return Err("Expected right parenthesis".to_string());
                                            },
                                        }
                                        
                                    },

                                    _ => {
                                        full_expression = Some(self.expression(Some(
                                            Expression::Binary(BinaryOperator::MemberAccess,
                                                Box::new(Expression::Identifier(ident)),
                                                Box::new(Expression::Identifier(member.clone()))
                                            )))?);
                                    },
                                }                                
                            },
                            Token::Arrow => {
                                self.head += 1;
                                let member = match self.tokens[self.head].clone() {
                                    Token::Word(ident) => {
                                        self.head += 1;
                                        ident
                                    },
                                    _ => {
                                        return Err("Expected identifier".to_string());
                                    },
                                };

                                match self.tokens[self.head].clone() {
                                    Token::LeftParen => {
                                        self.head += 1;

                                        let temp = self.expression(None)?;
                                        
                                        let arguments = match temp {
                                            Expression::Blank => {
                                                None
                                            },
                                            _ => {
                                                Some(Box::new(self.expression(None)?))
                                            },

                                        };

                                        match self.tokens[self.head] {
                                            Token::RightParen => {
                                                self.head += 1;
                                                full_expression = Some(self.expression(Some(Expression::CallMethod(true, ident, member.clone(), arguments)))?);
                                            },
                                            _ => {
                                                return Err("Expected right parenthesis".to_string());
                                            },
                                        }
                                        
                                    },

                                    _ => {
                                        full_expression = Some(self.expression(Some(
                                            Expression::Binary(BinaryOperator::PointerMemberAccess,
                                                Box::new(Expression::Identifier(ident)),
                                                Box::new(Expression::Identifier(member.clone()))
                                            )))?);
                                    },
                                }                                


                            },
                            _ => {
                                full_expression = Some(self.expression(Some(Expression::Identifier(ident)))?);
                            },
                        }
                    },
                    Token::Number(num) => {
                        self.head += 1;
                        full_expression = Some(Expression::Literal(Literal::Number(num)));
                    },
                    Token::String(string) => {
                        self.head += 1;
                        full_expression = Some(Expression::Literal(Literal::String(string)));
                    },
                    Token::Character(character) => {
                        self.head += 1;
                        full_expression = Some(Expression::Literal(Literal::Char(character)));
                    },
                    Token::True => {
                        self.head += 1;
                        full_expression = Some(Expression::Literal(Literal::Bool(true)));
                    },
                    Token::False => {
                        self.head += 1;
                        full_expression = Some(Expression::Literal(Literal::Bool(false)));
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
                        return Err(format!("Unexpected token {:?}", self.tokens[self.head]));
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


    pub fn parse(&mut self) -> Result<Header, String> {
        let mut header_statements = Vec::new();

        if self.tokens.len() == 0 {
            return Err("No tokens".to_string());
        }

        while self.tokens.len() != 0 {
            println!("Parsing token {:?}", self.tokens[0]);
            match self.tokens[0] {
                Token::Preprocessor(_) => {
                    header_statements.append(self.preprocessors()?.iter().map(|x| HeaderStatement::Preprocessor(x.clone())).collect::<Vec<HeaderStatement>>().as_mut());

                },
                Token::Typedef => {
                    self.head += 1;

                },
                Token::Struct => {
                    self.head += 1;

                },
                Token::Enum => {
                    self.head += 1;

                },
                Token::Union => {
                    self.head += 1;

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
                Token::Class => {
                    self.head += 1;

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

        assert!(parser.parse().is_ok(), "Failed to parse global variable");
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

        assert!(parser.parse().is_ok(), "Failed to parse global variables");
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
        println!("Tokens: {:?}", tokens);

        let mut parser = Parser::new(tokens);
        assert!(parser.parse().is_ok(), "Failed to parse global variable");
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

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        assert!(parser.parse().is_ok(), "Failed to parse global variable");
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

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        assert!(parser.parse().is_ok(), "Failed to parse global variable");
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

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        assert!(parser.parse().is_err(), "Parse global variable when it should have failed");
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

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        assert!(parser.parse().is_ok(), "Failed to parse global function pointer");
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

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        assert!(parser.parse().is_ok(), "Failed to parse global function pointer");
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

        println!("Tokens: {:?}", tokens);
        let mut parser = Parser::new(tokens);
        assert!(parser.parse().is_ok(), "Failed to parse global function pointer");
    }

    #[test]
    fn test_simple_function() {
        let input = "int main() { return 0; }\n";
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

}
