use crate::{Source, Store, Token};
use itertools::Itertools;
use std::fs;

pub struct Lexer<'a> {
    source: Source<'a>,
    store: Store,
}

impl<'a> Lexer<'a> {
    pub fn new(source: Source<'a>) -> Self {
        let mut lexer = Lexer {
            source: source,
            store: Store::new(),
        };
        lexer.run();

        lexer
    }

    fn run(&mut self) {
        loop {
            match self.lex() {
                Ok(Token::EOF) => {
                    self.store.push(Token::EOF);
                    break;
                }
                Ok(token) => self.store.push(token),
                Err(a) => panic!(a),
            }
        }
    }

    pub fn lex(&mut self) -> Result<Token, String> {
        match self.source.next() {
            Some(c) if c.is_whitespace() => self.lex(),
            Some(c) if c.is_digit(10) => self.int(),
            Some(c) if c.is_alphanumeric() => self.id(),
            Some(':') => match self.source.peek() {
                Some('=') => {
                    self.source.next();
                    Ok(Token::Assign)
                }
                _ => Ok(Token::Colon),
            },
            Some('\'') => self.string(),
            Some('<') => match self.source.peek() {
                Some('=') => {
                    self.source.next();
                    Ok(Token::LtEq)
                }
                Some('>') => {
                    self.source.next();
                    Ok(Token::Not)
                }
                _ => Ok(Token::Lt),
            },
            Some('>') => match self.source.peek() {
                Some('=') => {
                    self.source.next();
                    Ok(Token::LtEq)
                }
                _ => Ok(Token::Lt),
            },
            Some('=') => Ok(Token::Eq),
            Some('+') => Ok(Token::Plus),
            Some('-') => Ok(Token::Minus),
            Some('*') => Ok(Token::Mult),
            Some('/') => Ok(Token::Div),
            Some(',') => Ok(Token::Comma),
            Some('.') => Ok(Token::Dot),
            Some(';') => Ok(Token::SemiColon),
            Some('(') => Ok(Token::LParen),
            Some(')') => Ok(Token::RParen),
            // I do feel weird about having Some be an error and None be Ok
            Some(c) => Err(format!("Unknown char {} durning lexing", c)),
            None => Ok(Token::EOF),
        }
    }

    fn int(&mut self) -> Result<Token, String> {
        let init_val = match self.source.current() {
            Some(c) if c.is_digit(10) => Ok(c),
            _ => Err("Expected number"),
        }?
        .to_string();

        let ret_val = self
            .source
            .peeking_take_while(|c| c.is_digit(10))
            .fold(init_val.to_string(), |mut acc, next_char| {
                acc.push(next_char);
                acc
            })
            .parse::<i32>()
            .or(Err("Failed to parse number".to_string()));

        // I do wish there was a way to do this inline
        match ret_val {
            Ok(i) => Ok(Token::Int(i)),
            Err(e) => Err(e),
        }
    }

    fn string(&mut self) -> Result<Token, String> {
        match self.source.current() {
            Some('\'') => Ok(()),
            _ => Err("Expected '\''"),
        }?;

        let ret_string = self
            .source
            .by_ref()
            .peeking_take_while(|c| *c != '\'')
            .fold(String::new(), |mut acc, next| {
                acc.push(next);
                acc
            });

        match self.source.next() {
            Some('\'') => Ok(Token::String(ret_string)),
            _ => Err("Unterminated string".to_string()),
        }
    }

    fn id(&mut self) -> Result<Token, String> {
        let init_val: String = match self.source.current() {
            Some(c) if c.is_alphabetic() => Ok(c),
            _ => Err("Internal Lexer Error: Expected alphabetic character"),
        }?
        .to_string();
        let ret_id: String = self
            .source
            .by_ref()
            .peeking_take_while(|c| c.is_alphanumeric() || c == &'_')
            .fold(init_val, |mut acc: String, next_id: char| {
                acc.push(next_id);
                return acc;
            });

        match ret_id.to_uppercase().as_str() {
            "PROGRAM" => Ok(Token::Program),
            "BEGIN" => Ok(Token::Begin),
            "FUNCTION" => Ok(Token::Function),
            "END" => Ok(Token::End),
            "EOF" => Ok(Token::EOF),
            "VAR" => Ok(Token::Var),
            "PROCEDURE" => Ok(Token::Procedure),
            "IF" => Ok(Token::If),
            "THEN" => Ok(Token::Then),
            "ELSE" => Ok(Token::Else),
            "AND" => Ok(Token::And),
            "OR" => Ok(Token::Or),
            "FOR" => Ok(Token::For),
            "TO" => Ok(Token::To),
            "DO" => Ok(Token::Do),
            "WHILE" => Ok(Token::While),
            "INTEGER" => Ok(Token::IntT),
            "BOOLEAN" => Ok(Token::BoolT),
            "TRUE" => Ok(Token::Bool(true)),
            "FALSE" => Ok(Token::Bool(false)),
            _ => Ok(Token::ID(ret_id)),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.store.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign() {
        let source = Source::new("this := true");
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next(), Some(Token::ID(String::from("this"))));
        assert_eq!(lexer.next(), Some(Token::Assign));
        assert_eq!(lexer.next(), Some(Token::Bool(true)));
    }

    #[test]
    fn test_while() {
        let source = Source::new("while j<b do");
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next(), Some(Token::While));
        assert_eq!(lexer.next(), Some(Token::ID(String::from("j"))));
        assert_eq!(lexer.next(), Some(Token::Lt));
        assert_eq!(lexer.next(), Some(Token::ID(String::from("b"))));
        assert_eq!(lexer.next(), Some(Token::Do));
    }

    #[test]
    fn test_string() {
        let source = Source::new("'test'");
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next(), Some(Token::String(String::from("test"))));
    }

    #[test]
    fn test_eq() {
        let source = Source::new("'test' = 'test'");
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next(), Some(Token::String(String::from("test"))));
        assert_eq!(lexer.next(), Some(Token::Eq));
        assert_eq!(lexer.next(), Some(Token::String(String::from("test"))));
    }

    #[test]
    fn test_fib() {
        let file: String = fs::read_to_string("fib.pas").unwrap().clone();
        let source = Source::new(file.as_str());
        let mut lexer = Lexer::new(source);
        while let Some(a) = lexer.next() {
            if a == Token::EOF {
                return;
            } else {
                dbg!(a);
            }
        }
    }
}
