use crate::logos_lexer::Token;


#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    None,
    Header(Header),
    HeaderStatement(HeaderStatement),
    Preprocessor(Preprocessor),
    TypedefType(TypedefType),
    Typedef(Typedef),
    Struct(Struct),
    Union(Union),
    Enum(Enum),
    EnumMember(EnumMember),
    VariableValue(VariableValue),
    VariableArray(VariableArray),
    Variable(Variable),
    VariableList(VariableList),
    FunctionArgument(FunctionArgument),
    FunctionPrototype(FunctionPrototype),
    Function(Function),
    Class(Class),
    ClassMember(ClassMember),
    Method(Method),
    BaseType(BaseType),
    CompositeType(CompositeType),
    TypeType(TypeType),
    Type(Type),
    TypeModifier(TypeModifier),
    PrefixTypeModifier(PrefixTypeModifier),
    SuffixTypeModifier(SuffixTypeModifier),
    CodeBlock(CodeBlock),
    StatementList(StatementList),
    BlockOrStatement(BlockOrStatement),
    VariableListOrStatement(VariableListOrStatement),
    TypeOrExpression(TypeOrExpression),
    Statement(Statement),
    SwitchCase(SwitchCase),
    Expression(Expression),
    Literal(Literal),
    UnaryOperator(UnaryOperator),
    BinaryOperator(BinaryOperator),
    Initializer(Initializer),
    Designator(Designator),
    TaggedUnion(TaggedUnion),

}

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub statements: Vec<HeaderStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HeaderStatement {
    Preprocessor(Preprocessor),
    Typedef(Typedef),
    Struct(Struct),
    Union(Union),
    Enum(Enum),
    Variable(VariableList),
    FunctionPrototype(FunctionPrototype),
    Function(Function),
    Class(Class),
    TaggedUnion(TaggedUnion),
    Whitespace,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Preprocessor {
    pub value: String,
}


#[derive(Debug, Clone, PartialEq)]
pub enum TypedefType {
    Struct(Struct),
    Union(Union),
    Enum(Enum),
    Variable(Variable),
    FunctionPrototype(FunctionPrototype),
    Function(Function),
    Class(Class),
    Type(Type),
    PointerType(Type, usize),
    ArrayType(Type, String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Typedef {
    pub r#type: TypedefType,
    pub name: String,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub name: String,
    pub members: Vec<VariableList>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Union {
    pub name: String,
    pub members: Vec<VariableList>,
}

#[derive(Debug, Clone,  PartialEq)]
pub struct Enum {
    pub name: String,
    pub members: Vec<EnumMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumMember {
    pub name: String,
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TaggedUnion {
    pub name: String,
    pub members: Vec<TaggedUnionMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TaggedUnionMember {
    pub name: String,
    pub value: Option<Vec<VariableList>>,
}


#[derive(Debug, Clone,  PartialEq)]
pub enum VariableValue {
    Expression(Expression),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableArray {
    Size(Expression),
    NoSize,

}

#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
    BasicVar {
        name: String,
        pointer: usize,
        restrict: bool,
        array: Option<Vec<VariableArray>>,
        value: Option<VariableValue>,
    },
    FunctionPointer {
        return_type: Type,
        return_pointer: usize,
        pointer: usize,
        name: Option<String>,
        array: Option<Vec<VariableArray>>,
        arguments: Vec<FunctionArgument>,
    }, 
}


#[derive(Debug, Clone, PartialEq)]
pub enum VariableList {
    BasicVars {
        type_: Type,
        variables: Vec<Variable>,
    },
    FunctionPointer(Variable),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionArgument {
    FunctionPointer(Variable),
    Variable(Type, Variable),
    Type(Type,usize),
    Ellipsis,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionPrototype {
    pub return_type: Type,
    pub return_pointer: usize,
    pub name: String,
    pub arguments: Vec<FunctionArgument>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub inline: bool,
    pub static_: bool,
    pub return_type: Type,
    pub return_pointer: usize,
    pub name: String,
    pub arguments: Vec<FunctionArgument>,
    pub body: CodeBlock,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: String,
    pub parent: Option<String>,
    pub members: Vec<ClassMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassMember {
    Variables(Vec<VariableList>),
    Methods(Vec<Method>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    pub return_type: Type,
    pub name: String,
    pub arguments: Vec<FunctionArgument>,
    pub body: CodeBlock,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Types {
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Signed,
    Unsigned,
    Bool,
    Struct(String),
    Union(String),
    Enum(String),
    Pointer(Box<Type>),
    Array(Box<Type>, usize,Option<String>),
    FunctionPointer(Box<Type>, Vec<FunctionArgument>),
    Class(String),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BaseType {
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Signed,
    Unsigned,
    Bool,
}


#[derive(Debug, Clone, PartialEq)]
pub enum CompositeType {
    Mixed(Vec<BaseType>),
    Struct(String),
    Union(String),
    Enum(String),
    Pointer(Box<Type>, usize),
    Array(Box<Type>, usize),//type, pointer amount, array amount
    FunctionPointer(Box<Type>, Vec<FunctionArgument>),
    Identifier(String),
}


#[derive(Debug, Clone, PartialEq)]
pub enum TypeType {
    BaseType(String),
    CompositeType(CompositeType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    types: TypeType,
}


impl Type {
    pub fn from_token(token: Token) -> Result<Self,String> {
        match token {
            Token::Type(types) => Ok(Self {
                types: TypeType::BaseType(types),
            }),
            Token::Word(name) => Ok(Self {
                types: TypeType::CompositeType(CompositeType::Identifier(name)),
            }),
            _ => Err(format!("Expected type, got {:?}",token)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeModifier {
    Prefix(PrefixTypeModifier),
    Suffix(SuffixTypeModifier),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrefixTypeModifier {
    Const,
    Volatile,
    Restrict,
    Atomic,
    Static,
    Extern,
    Register,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SuffixTypeModifier {
    Restrict,
    Complex,
    Imaginary,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CodeBlock {
    Code(StatementList),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatementList {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockOrStatement {
    Block(CodeBlock),
    Statement(Statement),
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableListOrStatement {
    VariableList(VariableList),
    Statement(Statement),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeOrExpression {
    Type(Type,usize),
    Expression(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Preprocessor(Preprocessor),
    Comment(String),
    VariableList(VariableList),
    Expression(Expression),
    Return(Expression),
    If(Expression, Box<BlockOrStatement>),
    Else(Box<BlockOrStatement>),
    While(Expression, Box<BlockOrStatement>),
    DoWhile(Expression, Box<BlockOrStatement>),
    For(
        Option<Box<VariableListOrStatement>>,
        Option<Expression>,
        Option<Expression>,
        Box<BlockOrStatement>,
        ),
    Switch(Expression, Vec<SwitchCase>),
    Break,
    Continue,
    Goto(String),
    Label(String),
    Block(Box<CodeBlock>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCase {
    pub default: bool,
    pub expression: Option<Expression>,
    pub body: Box<BlockOrStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Blank,
    Identifier(String),
    Literal(Literal),
    Sizeof(TypeOrExpression),
    Alignof(Type, usize),
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Ternary(Box<Expression>, Box<Expression>, Box<Expression>),
    CallFunction(String, Option<Box<Expression>>),
    //CallMethod(bool, String, String, Option<Box<Expression>>),
    InitializerList(Box<Expression>),
    TaggedInitializer(String, Box<Expression>),
    //CompoundLiteral(Type, bool,Initializer),
    //InitializerList(Vec<Initializer>),
    StatementList(StatementList),
    Expression(Box<Expression>),
    Parentheses(Box<Expression>),
}

impl Expression {
    pub fn get_value(&self) -> Option<String> {
        match self {
            Expression::Literal(literal) => match literal {
                Literal::Number(number) => Some(number.clone()),
                Literal::Char(character) => Some(character.clone()),
                Literal::String(string) => Some(string.clone()),
                Literal::Bool(boolean) => Some(boolean.to_string()),
            },
            Expression::Identifier(name) => Some(name.clone()),
            _ => None,
        }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(String),
    Char(String),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
    BitwiseNot,
    LogicalNot,
    Dereference,
    AddressOf,
    PreIncrement,
    PreDecrement,
    PostIncrement,
    PostDecrement,
    Cast(Type,usize),
    MemberSet
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    LeftShift,
    RightShift,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LogicalAnd,
    LogicalOr,
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    LeftShiftAssign,
    RightShiftAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    Comma,
    MemberAccess,
    PointerMemberAccess,
    Index,
    ArrayAccess,

}


#[derive(Debug, Clone, PartialEq)]
pub enum Initializer {
    Expression(Box<Expression>),
    List(Vec<Initializer>),
    Designated(Designator, Box<Initializer>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Designator {
    Member(String),
    Index(Box<Expression>),
}


