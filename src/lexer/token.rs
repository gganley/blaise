#[derive(Debug, PartialEq)]
pub enum Token {
    Program,
    Begin,
    Function,
    End,
    EOF,
    Var,
    Procedure,
    If,
    Then,
    Else,
    And,
    Or,
    For,
    To,
    Do,
    While,
    Div,
    Mult,
    Plus,
    Minus,
    Assign,
    Not,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Eq,
    LParen,
    RParen,
    Comma,
    SemiColon,
    Colon,
    Dot,
    BoolT,
    IntT,
    ID(String),
    String(String),
    Int(i32),
    Bool(bool),
}

pub struct Store {
    tokens: Vec<Token>,
}

impl Store {
    pub fn new() -> Self {
        Store { tokens: Vec::new() }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(0)
    }

    pub fn push(&mut self, item: Token) {
        self.tokens.push(item)
    }

    pub fn peek_ahead(&self, n: usize) -> Option<&Token> {
        self.tokens.get(n)
    }
}

impl Iterator for Store {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tokens.is_empty() {
            None
        } else {
            Some(self.tokens.remove(0))
        }
    }
}
