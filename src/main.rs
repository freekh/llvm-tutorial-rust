use std::io::*;

enum Token {
  Eof,
  Number(f64),
  Identifier(String),
}

fn getToken() {

}

fn main() {
  let reader = BufReader::new(stdin());
  let buf: []
  while (reader.read(buf).is_ok()) {
    getToken();
  }
}