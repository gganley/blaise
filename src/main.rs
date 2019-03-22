fn main() {
    println!("{}", "hello");
    let _ = chomp("Hello");
}

/// <varpart> ::= VAR <vars>+
pub enum Varpart {
    Varpart(Vec<Vars>)
}

/// <vars> ::= <varstatement>+
pub enum Vars {
    Vars(Vec<Vardef>)
}

/// <vardef> ::= <name> {, <name>} : <type>
pub enum Vardef {
    Vardef(Vec<String>, Type)
}

/// <type> ::= BOOLEAN | INTEGER
pub enum Type {
    Boolean,
    Integer
}

/// <expression> ::= <atom> | <subexpression> | <operation>
pub enum Expression {
    Atom(Atomic),
    SubExpr(Box<Expression>),
    Operation(Op, Box<Expression>, Box<Expression>)
}

/// <statement> ::= <if-statement> | <for-statement> |
///                 <while-statement> | <assignment-statement> |
///                 <procedure-call> | <writeln-statement>
pub enum Statement {
    /// <if-statement> ::= IF <expression> THEN BEGIN [<statements>] END; [<else-clause>]
    If(Expression, Option<Vec<Statement>>, Option<Vec<Statement>>),
    /// <for-statement> ::= FOR <name> := <integer> TO <integer> DO BEGIN [<statements>] END; 
    For(String, i32, i32, Option<Vec<Statement>>),
    /// <while-statement> ::= WHILE <expression> DO BEGIN [<statements>] END;
    While(Expression, Option<Vec<Statement>>),
    /// <assignment-statement> ::= <name> := <expression> ;
    Assign(String, Expression),
    /// <procedure> ::= PROCEDURE <name> [<parampart>] ; [<varpart>] BEGIN [<statements>] END;
    Procedure(String, Option<Vec<Vardef>>, Option<Vec<Vardef>>, Option<Vec<Statement>>),
    /// <writeln-statement> ::= WRITELN ( <atom> ) ;
    Writeln(Vec<Atomic>)
}

/// <operator> ::= + | - | = | <> | < | > | <= | =>
pub enum Op {
    Plus,
    Sub,
    Eq,
    Fish,
    Lt,
    Gt,
    Lte,
    Gte
}

/// <atom> ::= <name> | <integer> | <boolean>
pub enum Atomic {
    /// <name> ::= <letter> {<letter>} {<number>}
    Name(String),
    Integer(i32),
    Boolean(bool)
}

fn chomp(stream: &str) -> Option<(Expression, &str)> {
    Some((Expression::Atom(Atomic::Name(stream.into())), stream))
}
