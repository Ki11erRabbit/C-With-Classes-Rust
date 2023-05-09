

#[derive(Debug, Clone)]
pub struct Header {
    pub preprocessor: Vec<String>,
    pub typedefs: Vec<Typedef>,
    pub structs: Vec<Struct>,
    pub unions: Vec<Union>,
    pub enums: Vec<Enum>,
    pub variables: Vec<VariableList>,
    pub function_prototypes: Vec<FunctionPrototype>,
    pub functions: Vec<Function>,
    pub classes: Vec<Class>,
}

#[derive(Debug, Clone,)]
pub enum Preprocessor {
    IncludeSystem(String),
    IncludeLocal(String),
    Define(String, String),
    DefineFunction(String, Vec<String>, String),
    Undef(String),
    If(String, Vec<String>),
    Ifdef(String, Vec<String>),
    Ifndef(String, Vec<String>),
    Else(Vec<String>),
    Endif,
    Pragma(Vec<String>),
    Error(String),
    Hash(String),
    DoubleHash(String, usize),
}


#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Typedef {
    pub r#type: TypedefType,
    pub name: String,
}


#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub members: Vec<Variable>,
}

#[derive(Debug, Clone)]
pub struct Union {
    pub name: String,
    pub members: Vec<Variable>,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub members: Vec<EnumMember>,
}

#[derive(Debug, Clone)]
pub struct EnumMember {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub enum VariableValue {
    Expression(Expression),
    String(String),
}

#[derive(Debug, Clone)]
pub enum VariableArray {
    Size(Expression),
    NoSize,

}

#[derive(Debug, Clone)]
pub enum Variable {
    BasicVar {
        name: String,
        pointer: usize,
        array: Option<Vec<VariableArray>>,
        value: Option<VariableValue>,
    },
    FunctionPointer {
        return_type: Type,
        name: String,
        arguments: Vec<FunctionArgument>,
    }, 
}

#[derive(Debug, Clone)]
pub enum VariableList {
    BasicVars {
        type_: Type,
        variables: Vec<Variable>,
    },
    FunctionPointer(Variable),
}

#[derive(Debug, Clone)]
pub enum FunctionArgument {
    Variable(Variable),
    Type(Type),
    Ellipsis,
}

#[derive(Debug, Clone)]
pub struct FunctionPrototype {
    pub return_type: String,
    pub name: String,
    pub arguments: Vec<FunctionArgument>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub inline: bool,
    pub static_: bool,
    pub return_type: (Type,TypeModifier),
    pub name: String,
    pub arguments: Vec<FunctionArgument>,
    pub body: CodeBlock,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub parent: Option<String>,
    pub members: Vec<ClassMember>,
}

#[derive(Debug, Clone)]
pub enum ClassMember {
    Variables(Vec<VariableList>),
    Methods(Vec<Method>),
}

#[derive(Debug, Clone)]
pub struct Method {
    pub return_type: Type,
    pub name: String,
    pub arguments: Vec<FunctionArgument>,
    pub body: CodeBlock,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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


#[derive(Debug, Clone)]
pub enum TypeType {
    BaseType(BaseType),
    CompositeType(CompositeType),
}

#[derive(Debug, Clone)]
pub struct Type {
    types: TypeType,
    modifiers: Vec<TypeModifier>,
}

#[derive(Debug, Clone)]
pub enum TypeModifier {
    Prefix(PrefixTypeModifier),
    Suffix(SuffixTypeModifier),
}

#[derive(Debug, Clone)]
pub enum PrefixTypeModifier {
    Const,
    Volatile,
    Restrict,
    Atomic,
    Static,
    Extern,
    Register,
}

#[derive(Debug, Clone)]
pub enum SuffixTypeModifier {
    Restrict,
    Complex,
    Imaginary,
}

#[derive(Debug, Clone)]
pub enum CodeBlock {
    Code(StatementList),
}

#[derive(Debug, Clone)]
pub struct StatementList {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum BlockOrStatement {
    Block(CodeBlock),
    Expression(Statement),
}

#[derive(Debug, Clone)]
pub enum VariableListOrStatement {
    VariableList(VariableList),
    Statement(Statement),
}

#[derive(Debug, Clone)]
pub struct TypeOrExpression {
    pub type_: Type,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Preprocessor(Vec<Preprocessor>),
    VariableList(Vec<VariableList>),
    Expression(Expression),
    Return(Expression),
    If(Expression, Box<BlockOrStatement>, Option<Box<Statement>>),
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

#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub default: bool,
    pub expression: Option<Expression>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    Sizeof(TypeOrExpression),
    Alignof(Type, usize),
    Cast(Type,bool, Box<Expression>),
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Ternary(Box<Expression>, Box<Expression>, Box<Expression>),
    Call(String, Option<Vec<Expression>>),
    CompoundLiteral(Type, bool,Initializer),
    InitializerList(Vec<Initializer>),
    StatementList(StatementList),
    Expression(Box<Expression>),
    Parentheses(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(String),
    Char(String),
    String(String),
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
    BitwiseNot,
    Dereference,
    AddressOf,
    PreIncrement,
    PreDecrement,
    PostIncrement,
    PostDecrement,
}

#[derive(Debug, Clone)]
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

}


#[derive(Debug, Clone)]
pub enum Initializer {
    Expression(Box<Expression>),
    List(Vec<Initializer>),
    Designated(Designator, Box<Initializer>),
}

#[derive(Debug, Clone)]
pub enum Designator {
    Member(String),
    Index(Box<Expression>),
}


