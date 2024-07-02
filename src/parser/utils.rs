use crate::lexer::Token;

#[derive(Debug, Clone)]
pub struct Tokens {
  pub len: usize,
  pub row: Vec<Token>,
  pub current: usize
}

impl Tokens {
  pub fn from(tokens: Vec<Token>) -> Tokens {
    Tokens { 
      len: tokens.len(),
      row: tokens,
      current: 0
    }
  }
  
  pub fn next(&mut self) -> Option<Token> {
    if self.current == self.len { return None }
    self.current += 1;
    Some(self.row[self.current - 1].clone())
  }

  pub fn after(&mut self) -> Option<Token> {
    if self.current == self.len { return None }
    Some(self.row[self.current].clone())
  }

  pub fn back(&mut self) {
    self.current -= 1;
  }

  pub fn before(&mut self) -> Option<Token> {
    if self.current <= 1 { return None }
    Some(self.row[self.current - 2].clone())
  }
}