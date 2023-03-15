#[derive(Debug)]
pub enum Node {
    Program(ProgramNode),
    Statement(StatementNode),
    Identifier(IdentifierNode),
    Shape(ShapeNode),
    Property(PropertyNode),
    NumberLiteral(NumberLiteralNode),
    StringLiteral(StringLiteralNode),
    BooleanLiteral(BooleanLiteralNode),
    BinaryExpression(BinaryExpressionNode),
    FunctionCall(FunctionCallNode),
    VariableDeclaration(VariableDeclarationNode),
    VariableAssignment(VariableAssignmentNode),
    IfStatement(IfStatementNode),
    WhileLoop(WhileLoopNode),
    ForLoop(ForLoopNode),
    Block(BlockNode),
}

#[derive(Debug)]
pub struct ProgramNode {
    pub statements: Vec<Node>,
}

#[derive(Debug)]
pub struct StatementNode {
    pub kind: StatementKind,
}

#[derive(Debug)]
pub enum StatementKind {
    Expression(Box<Node>),
    Return(Box<Node>),
}

#[derive(Debug)]
pub struct IdentifierNode {
    pub name: String,
}

#[derive(Debug)]
pub struct ShapeNode {
    pub kind: ShapeKind,
    pub properties: Vec<Node>,
}

#[derive(Debug)]
pub enum ShapeKind {
    Circle,
    Rectangle,
    Polygon,
    SVG,
}

#[derive(Debug)]
pub struct PropertyNode {
    pub name: String,
    pub value: Box<Node>,
}

#[derive(Debug)]
pub struct NumberLiteralNode {
    pub value: f64,
}

#[derive(Debug)]
pub struct StringLiteralNode {
    pub value: String,
}

#[derive(Debug)]
pub struct BooleanLiteralNode {
    pub value: bool,
}

#[derive(Debug)]
pub struct BinaryExpressionNode {
    pub left: Box<Node>,
    pub operator: BinaryOperator,
    pub right: Box<Node>,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    And,
    Or,
}

#[derive(Debug)]
pub struct FunctionCallNode {
    pub name: String,
    pub arguments: Vec<Node>,
}

#[derive(Debug)]
pub struct VariableDeclarationNode {
    pub name: String,
    pub initializer: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct VariableAssignmentNode {
    pub name: String,
    pub value: Box<Node>,
}

#[derive(Debug)]
pub struct IfStatementNode {
    pub condition: Box<Node>,
    pub then_block: Box<Node>,
    pub else_block: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct WhileLoopNode {
    pub condition: Box<Node>,
    pub block: Box<Node>,
}

#[derive(Debug)]
pub struct ForLoopNode {
    pub variable: String,
    pub initializer: Box<Node>,
    pub condition: Box<Node>,
    pub increment: Box<Node>,
    pub block: Box<Node>,
}

#[derive(Debug)]
pub struct BlockNode {
    pub statements: Vec<Node>,
}
