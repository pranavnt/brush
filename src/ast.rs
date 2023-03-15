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
    UnaryExpression(UnaryExpressionNode),
    FunctionCall(FunctionCallNode),
    VariableDeclaration(VariableDeclarationNode),
    VariableAssignment(VariableAssignmentNode),
    IfStatement(IfStatementNode),
    WhileLoop(WhileLoopNode),
    ForLoop(ForLoopNode),
    Block(BlockNode),
}

pub struct ProgramNode {
    pub statements: Vec<Node>,
}

pub struct StatementNode {
    pub kind: StatementKind,
}

pub enum StatementKind {
    Expression(Box<Node>),
    Return(Box<Node>),
}

pub struct IdentifierNode {
    pub name: String,
}

pub struct ShapeNode {
    pub kind: ShapeKind,
    pub properties: Vec<Node>,
}

pub enum ShapeKind {
    Circle,
    Rectangle,
    Polygon,
    SVG,
}

pub struct PropertyNode {
    pub name: String,
    pub value: Node,
}

pub struct NumberLiteralNode {
    pub value: f64,
}

pub struct StringLiteralNode {
    pub value: String,
}

pub struct BooleanLiteralNode {
    pub value: bool,
}

pub struct BinaryExpressionNode {
    pub left: Box<Node>,
    pub operator: BinaryOperator,
    pub right: Box<Node>,
}

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

pub struct UnaryExpressionNode {
    pub operator: UnaryOperator,
    pub operand: Box<Node>,
}

pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
}

pub struct FunctionCallNode {
    pub name: String,
    pub arguments: Vec<Node>,
}

pub struct VariableDeclarationNode {
    pub name: String,
    pub initializer: Option<Box<Node>>,
}

pub struct VariableAssignmentNode {
    pub name: String,
    pub value: Box<Node>,
}

pub struct IfStatementNode {
    pub condition: Box<Node>,
    pub then_block: Box<Node>,
    pub else_block: Option<Box<Node>>,
}

pub struct WhileLoopNode {
    pub condition: Box<Node>,
    pub block: Box<Node>,
}

pub struct ForLoopNode {
    pub variable: String,
    pub initializer: Box<Node>,
    pub condition: Box<Node>,
    pub increment: Box<Node>,
    pub block: Box<Node>,
}

pub struct BlockNode {
    pub statements: Vec<Node>,
}
