use std::io::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  Def,
  Extern,
  Number(f64),
  Identifier(String),
  Char(char),
}

pub struct Lexer {
  cont: Box<FnMut() -> Chars<Stdin>>,
}

impl Lexer {
  pub fn new(cont: Box<FnMut() -> Chars<Stdin>>) -> Lexer {
    Lexer {
      cont: cont,
    }
  }
}

impl Iterator for Lexer {
  type Item = Token;

  fn next(&mut self) -> Option<Token> {
    let chars = (self.cont)().map(|r| r.unwrap());

    let mut non_whitespace_chars = chars
      .skip_while(|&c| c.is_whitespace())
      .peekable();
    if let Some(&peek_char) = non_whitespace_chars.peek() {
      if peek_char.is_alphabetic() {
        let identifier_str: String = non_whitespace_chars
          .take_while(|&c| c.is_alphanumeric())
          .collect();
        Some(
          match identifier_str.as_str() {
            "def" => Token::Def,
            "extern" => Token::Extern,
            _ => Token::Identifier(identifier_str),
          }
        )
      } else if peek_char.is_digit(10) || peek_char == '.' {
        let num_str: String = non_whitespace_chars
          .take_while(|&c| c.is_digit(10) || c == '.')
          .collect();
        Some(
          Token::Number(num_str.parse::<f64>().unwrap()) // fails catastrophically if not possible, exercise for the reader
        )
      } else if peek_char == '#' {
         let _comment: String = non_whitespace_chars
           .take_while(|&c| c != '\n' && c != '\r')
           .collect();
         self.next()
      } else {
        non_whitespace_chars.next().map(|last_char| {
          Token::Char(last_char)
        })
      }
    } else {
      None
    }
  }
}