pub enum Frame {
    ShapeDeclaration,
    ShapeDraw,
    FunctionCall,
    FunctionDeclaration,
    ForLoop,
    WhileLoop,
}

pub struct ShapeDrawFrame {
    pub name: String,
    pub kind: ShapeKind,
    pub properties: Vec<PropertyNode>,
    pub statements: Vec<Node>,
}

pub enum ShapeKind {
    Circle,
    Rectangle,
    Polygon,
    SVG,
}

pub struct CallStack {
    pub frames: Vec<Frame>,
}

impl CallStack {
    pub fn new() -> CallStack {
        CallStack {
            frames: Vec::new(),
        }
    }

    pub fn push(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub fn pop(&mut self) -> Option<Frame> {
        self.frames.pop()
    }

    pub fn peek(&self) -> Option<&Frame> {
        self.frames.last()
    }
}