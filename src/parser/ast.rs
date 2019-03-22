pub struct Program(pub Variable, pub Block);
pub struct Block(pub Vec<Decl>, pub Vec<Stmt>);
pub enum Decl {
    ProcDecl(Vec<ProcDecl>),
    FnDecl(Vec<FnDecl>),
    VarDecl(Vec<VarDecl>),
    Epsilon,
}
pub struct ProcDecl(pub String, pub ParameterList, pub Block);
pub struct FnDecl(pub String, pub ParameterList, pub Block, pub Type);
pub struct ParameterList(pub Vec<Parameter>);
pub struct Parameter(pub Vec<String>, pub Type);
pub struct VarDecl(pub Vec<String>, pub Type);
pub enum Type {
    Integer,
    Boolean,
}

pub struct StmtBlock(pub Vec<Stmt>);
pub enum Stmt {
    Nested(Stmt)https://github.com/tylerlaberge/rascal/blob/master/src/parser/parser.rhttps://github.com/tylerlaberge/rascal/blob/master/src/parser/parser.rss,
    Assignment(Assignment),
    IfStatement(IfStatement),
    FunctionCall(FunctionDecl),
}

pub enum IfStmt {
    If(Expr, StmtBlock),
    IfElse(Expr, StmtBlock, StmtBlock),
}

pub struct Var(pub String);

pub struct FnCall(pub Var, pub FnParameters);

pub struct FnParameters(pub Vec<Expr>);

pub enum Expr {
    Group(Group),
    BinOp(BinOpExpr),
    FnCall(FnCall),
    Literal(Literal),
    Var(Var),
}

pub enum BinOpExpr {
    Div,
    Mult,
    Minus,
    Plus,
    And,
    Or,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Eq,
    NotEq,
}

pub struct Group(pub Expr);

pub struct Assignment(pub Variable, pub Expr);

pub enum Literal {
    Int(i32),
    Boolean(bool),
}
