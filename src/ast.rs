#[derive(Debug, Clone)]
pub enum Node {
    Program(ProgramNode),
    Statement(StatementNode),
    Identifier(IdentifierNode),
    Shape(ShapeNode),
    Property(PropertyNode),
    NumberLiteral(NumberLiteralNode),
    StringLiteral(StringLiteralNode),
    BooleanLiteral(BooleanLiteralNode),
    TupleLiteral(TupleLiteralNode),
    BinaryExpression(BinaryExpressionNode),
    FunctionCall(FunctionCallNode),
    VariableDeclaration(VariableDeclarationNode),
    VariableAssignment(VariableAssignmentNode),
    IfStatement(IfStatementNode),
    WhileLoop(WhileLoopNode),
    ForLoop(ForLoopNode),
    Block(BlockNode),
}

#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub statements: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct StatementNode {
    pub kind: StatementKind,
}

#[derive(Debug, Clone)]
pub enum StatementKind {
    DrawShape(String, Vec<PropertyNode>),
    Expression(Box<Node>),
    Return(Box<Node>),
    Shift(Box<Node>, Box<Node>),
    HueShift(Box<Node>),
    Stretch(Box<Node>, Box<Node>),
    Rotate(Box<Node>),
    RotateTo(Box<Node>),
    RotateAbout(Box<Node>, Box<Node>, Box<Node>),
    Reflect(Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    Warp(Box<Node>, Box<Node>),
}

#[derive(Debug, Clone)]
pub struct IdentifierNode {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ShapeNode {
    pub name: String,
    pub kind: ShapeKind,
    pub statements: Vec<Vec<Node>>,
}

#[derive(Debug, Clone)]
pub enum ShapeKind {
    Circle,
    Rectangle,
    Polygon,
    SVG,
}

#[derive(Debug, Clone)]
pub struct PropertyNode {
    pub name: String,
    pub value: Box<Node>,
}

#[derive(Debug, Clone)]
pub struct NumberLiteralNode {
    pub value: f32,
}

#[derive(Debug, Clone)]
pub struct TupleLiteralNode {
    pub values: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct StringLiteralNode {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct BooleanLiteralNode {
    pub value: bool,
}

#[derive(Debug, Clone)]
pub struct BinaryExpressionNode {
    pub left: Box<Node>,
    pub operator: BinaryOperator,
    pub right: Box<Node>,
}

#[derive(Debug, Clone)]
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
    Not,
    Or,
}

#[derive(Debug, Clone)]
pub struct FunctionCallNode {
    pub name: String,
    pub arguments: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct VariableDeclarationNode {
    pub name: String,
    pub initializer: Option<Box<Node>>,
}

#[derive(Debug, Clone)]
pub struct VariableAssignmentNode {
    pub name: String,
    pub value: Box<Node>,
}

#[derive(Debug, Clone)]
pub struct IfStatementNode {
    pub condition: Box<Node>,
    pub then_block: Box<Node>,
    pub else_block: Option<Box<Node>>,
}

#[derive(Debug, Clone)]
pub struct WhileLoopNode {
    pub condition: Box<Node>,
    pub block: Box<Node>,
}

#[derive(Debug, Clone)]
pub struct ForLoopNode {
    pub variable: String,
    pub initializer: Box<Node>,
    pub condition: Box<Node>,
    pub increment: Box<Node>,
    pub block: Box<Node>,
}

#[derive(Debug, Clone)]
pub struct BlockNode {
    pub statements: Vec<Node>,
}