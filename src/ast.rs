#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOpr {
    Not,
    Minus,
    Length,
    NoUnary,
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOpr {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Concat,
    Eq,
    NE,
    LT,
    LE,
    GT,
    GE,
    And,
    Or,
    NoBinary,
}

/// Expressions
#[derive(Debug, Clone)]
pub enum Expr {
    Nil,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Dots,
    Ident(String),
    UnaryOp(UnaryOpr, Box<ExprNode>),
    BinaryOp(BinaryOpr, Box<ExprNode>, Box<ExprNode>),
    FuncCall(Box<ExprNode>, Vec<ExprNode>),
    MethodCall(Box<ExprNode>, String, Vec<ExprNode>),
    AttrGet(Box<ExprNode>, Box<ExprNode>),
    Table(Vec<Field>),
    Function(ParList, Vec<StmtNode>),
}

/// A wrapper that stores the expression and its span
#[derive(Debug, Clone)]
pub struct ExprNode {
    pub expr: Expr,
    pub span: Span,
}

impl ExprNode {
    pub fn new(expr: Expr, span: (u32, u32)) -> Self {
        Self {
            expr,
            span: Span::new(span.0, span.1),
        }
    }
}

/// Table fields
#[derive(Debug, Clone)]
pub struct Field {
    pub key: Option<ExprNode>,
    pub val: ExprNode,
}

impl Field {
    pub fn new(key: Option<ExprNode>, val: ExprNode) -> Self {
        Self { key, val }
    }
}

/// Parameter list for functions
#[derive(Debug, Clone)]
pub struct ParList {
    pub names: Vec<String>,
    pub varargs: bool,
}

impl ParList {
    pub fn new() -> Self {
        Self {
            names: vec![],
            varargs: false,
        }
    }

    pub fn set_names(&mut self, names: Vec<String>) {
        self.names = names;
    }

    pub fn set_vargs(&mut self, varargs: bool) {
        self.varargs = varargs;
    }
}

/// Statements
#[derive(Debug, Clone)]
pub enum Stmt {
    Break,
    Return(Vec<ExprNode>),
    Assign(Vec<ExprNode>, Vec<ExprNode>),
    LocalAssign(Vec<String>, Vec<ExprNode>),
    FuncCall(ExprNode),
    MethodCall(ExprNode),
    DoBlock(Vec<StmtNode>),
    If(IfThenElse),
    While(ExprNode, Vec<StmtNode>),
    Repeat(ExprNode, Vec<StmtNode>),
    NumberFor(NumberFor),
    GenericFor(GenericFor),
    FuncDef(FuncDef),
    MethodDef(MethodDef),
}

/// A wrapper storing a statement and its span
#[derive(Debug, Clone)]
pub struct StmtNode {
    pub stmt: Stmt,
    pub span: Span,
}

impl StmtNode {
    pub fn new(stmt: Stmt, span: (u32, u32)) -> Self {
        Self {
            stmt,
            span: Span::new(span.0, span.1),
        }
    }
}

/// If-then-else structure
#[derive(Debug, Clone)]
pub struct IfThenElse {
    pub cond: ExprNode,
    pub then_branch: Vec<StmtNode>,
    pub else_branch: Vec<StmtNode>,
}

impl IfThenElse {
    pub fn new(cond: ExprNode, then_branch: Vec<StmtNode>, else_branch: Vec<StmtNode>) -> Self {
        Self {
            cond,
            then_branch,
            else_branch,
        }
    }

    pub fn set_els(&mut self, els: Vec<StmtNode>) {
        self.else_branch = els;
    }
}

/// Numeric for-loop
#[derive(Debug, Clone)]
pub struct NumberFor {
    pub var: String,
    pub init: ExprNode,
    pub limit: ExprNode,
    pub step: ExprNode,
    pub body: Vec<StmtNode>,
}

impl NumberFor {
    pub fn new(
        var: String,
        init: ExprNode,
        limit: ExprNode,
        step: ExprNode,
        body: Vec<StmtNode>,
    ) -> Self {
        Self {
            var,
            init,
            limit,
            step,
            body,
        }
    }
}

/// Generic for-loop
#[derive(Debug, Clone)]
pub struct GenericFor {
    pub names: Vec<String>,
    pub exprs: Vec<ExprNode>,
    pub body: Vec<StmtNode>,
}

impl GenericFor {
    pub fn new(names: Vec<String>, exprs: Vec<ExprNode>, body: Vec<StmtNode>) -> Self {
        Self { names, exprs, body }
    }
}

/// Function definition
#[derive(Debug, Clone)]
pub struct FuncDef {
    pub name: ExprNode,
    pub body: ExprNode,
}

impl FuncDef {
    pub fn new(name: ExprNode, body: ExprNode) -> Self {
        Self { name, body }
    }
}

/// Method definition
#[derive(Debug, Clone)]
pub struct MethodDef {
    pub obj: ExprNode,
    pub method: String,
    pub body: ExprNode,
}

impl MethodDef {
    pub fn new(obj: ExprNode, method: String, body: ExprNode) -> Self {
        Self { obj, method, body }
    }
}
