#![feature(io)]
extern crate inkwell;

mod lexer;
mod ast;
mod parser;

use std::io::*;
use lexer::{Token, Lexer};
use parser::{Error, Parser};

fn main() {
    // TODO: the closure is bad, but the best solution that came up. consider working more on this
  let lexer = Lexer::new(Box::new(|| { stdin().chars() }));
  let mut parser = Parser {
    lexer: Box::new(lexer.peekable()),
  };

  loop {
    match parser.peek() {
      None => break,
      Some(&Token::Def) => parser.def(),
      Some(&Token::Extern) => parser.ext(),
      // Some(&Token::Char(';')) => parser.skip(),
      _ => parser.expr(),
    };
  }
}