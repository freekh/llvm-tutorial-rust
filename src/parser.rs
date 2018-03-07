use ast;
use lexer::{Lexer, Token };
use std::iter::Peekable;

pub struct Error {
  pub id: String,
}

pub struct Parser {
  pub lexer: Box<Peekable<Lexer>>,
}

impl Parser {
  pub fn peek(&mut self) -> Option<&Token> {
    self.lexer.peek()
  }

  pub fn skip(&mut self) {
    self.lexer.next();
  }

  pub fn ext(&mut self) -> Result<ast::Function, Error> {
    Err(Error {
      id: "not-implemented".to_string(),
    })
  }

  pub fn expr(&mut self) -> Result<ast::Function, Error> {
    Err(Error {
      id: "not-implemented".to_string(),
    })
  }

  pub fn def(&mut self) -> Result<ast::Function, Error> {
    match self.lexer.next() {
      Some(Token::Def) => println!("Deffi-def"),
      _ => println!("ops"),
    }
    Err(Error {
      id: "not-implemented".to_string(),
    })
  }
}