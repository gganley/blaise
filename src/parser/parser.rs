use self::ast;

pub enum Precedence {
    Parens = 5,
    Product = 4,
    Sum = 3,
    Comp = 2,
    Terms = 1,
}

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser { lexer }
    }

    pub fn program(&mut self) -> Result<ast::Program, String> {
        match self.lexer.next() {
            Some(Token::Program) => {}
            _ => return Err(String::from("Expected 'PROGRAM'")),
        };

        let variable = self.variable()?;

        match self.lexer.next() {
            Some(Token::SemiColon) => {}
            _ => return Err(String::from("Expected ';'")),
        };

        let block = self.block()?;

        match self.lexer.next() {
            Some(Token::Dot) => {}
            _ => return Err(String::from("Expected '.'")),
        };

        Ok(ast::Program(variable, block))
    }

    pub fn block(&mut self) -> Result<ast::Block, String> {
        Ok(self.decls()?, self.stmt_block()?)
    }

    pub fn decls(&mut self) -> Result<Vec<ast::Decl>, String> {}

    pub fn var_decl(&mut self) -> Result<ast::VarDecl, String> {
        unimplemented!()
    }

    pub fn type_type(&mut self) -> Result<ast::Type, String> {
        unimplemented!()
    }

    pub fn proc_decl(&mut self) -> Result<ast::ProcDecl, String> {
        unimplemented!()
    }

    pub fn fn_decl(&mut self) -> Result<ast::FnDecl, String> {
        unimplemented!()
    }

    pub fn parameter_list(&mut self) -> Result<ast::ParameterList, String> {
        unimplemented!()
    }

    pub fn parameter(&mut self) -> Result<ast::Parameter, String> {
        unimplemented!()
    }

    pub fn stmt_block(&mut self) -> Result<ast::StmtBlock, String> {
        unimplemented!()
    }

    pub fn stmt(&mut self) -> Result<ast::Stmt, String> {
        unimplemented!()
    }

    pub fn if_stmt(&mut self) -> Result<ast::IfStmt, String> {
        unimplemented!()
    }

    pub fn assignment_stmt(&mut self) -> Result<ast::Assignement, String> {
        unimplemented!()
    }

    pub fn var(&mut self) -> Result<ast::Var, String> {
        unimplemented!()
    }

    pub fn fn_call(&mut self) -> Result<ast::FnCall, String> {
        unimplemented!()
    }

    pub fn fn_parameters(&mut self) -> Result<ast::FnParameters, String> {
        unimplemented!()
    }

    pub fn expr(&mut self) -> Result<ast::Expr, String> {
        unimplemented!()
    }
}
