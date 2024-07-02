pub mod utils;

use std::path;
use colored::*;

use utils::Bytes;
use crate::error;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
  BinaryOperator(&'static str),
  Identifier,
  Keyword,
  Type,
  Operator(&'static str),
  LiteralQuoute,
  LiteralChar,
  Parenthesis(bool),
  Brace(bool),
  Bracket(bool),
  Undefined,
  AnyType,
  Invalid
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub location: (usize, usize),
  pub content: String,
  pub type_: TokenType
}

fn is_operator(c: char) -> bool {
  ['-', '/', '*', '%', '=', '<', '.', ',', '>', '!', '|', ';', '&', ':', '^', '~', '$', '(', ')', '{', '}', '[', ']', '?', ':'].contains(&c)
}

fn is_double_operator(c: char) -> bool {
  ['-', '/', '*', '=', '<', '>', '!', '|', '&', ':', '{'].contains(&c)
}

fn is_keyword(buffer: &String) -> bool {
  [
    "undefined", "nil",
    "var",       "const",
    "if",        "else",
    "try",       "catch",
    "fn",        "pub", 
    "do",        "while",
    "for",       "continue",
    "loop",      "stop",
    "break",     "then",
    "switch",    "case",
    "default",   "unrachable",
    "into",      "enum",
    "union",     "type",
    "struct",    "impl",
    "return",    "await",
    "as",        "in"
  ].contains(&buffer.as_str())
}

fn is_invalid(buffer: &String) -> bool {
  [
    "void", "array",
    "string"
  ].contains(&buffer.as_str())
}

fn is_identifier(buffer: &String) -> bool {
  let chars: Vec<char> = buffer.chars().collect();

  for (i, ch) in chars.iter().enumerate() {
    if i == 0 && (ch.is_alphabetic() || ch == &'@') { continue; }
    if i >= 1 && ch.is_alphanumeric() { continue; }
    return false;
  }
  
  true
}

fn gen_error(file: &mut (Bytes, String, String), content: String, location: (usize, usize)) -> String {
  let (_, name) =  file.2.rsplit_once(path::MAIN_SEPARATOR).unwrap();
  let lines: Vec<&str> = file.1.lines().collect();
  format!(
    "In {name} {}:{} ocurred a lexical error:\n  {} {}\n  {}{}", 
    location.0, location.1, format!("{} |", location.0).blue(),
    lines[location.0 - 1],
    " ".repeat(format!("{} |", location.0).len() + location.1), "^".repeat(content.len()).green()
  )
}

pub fn parse(file: &mut (Bytes, String, String)) -> Vec<Token>{
  let mut bytes: Bytes = file.0.clone();

  let mut tokens: Vec<Token> = Vec::new();
  let mut buffer: String = String::new();

  let (mut line, mut col) = (1, 0);

  loop {
    col += 1;

    let c = match bytes.getc() {
      Some(c) => c,
      None => break
    };

    if c == '\r' { 
      col -= 1; 
      continue; 
    }

    if c == '\n' { 
      col = 0;
      line += 1; 
      continue; 
    }

    if !c.is_alphanumeric() && c != '@' {
      
      if !buffer.is_empty() {
        
        tokens.push(Token {
          location: (line, col),
          content: buffer.clone(),
          type_: if is_keyword(&buffer) {
            TokenType::Keyword
          } else if is_invalid(&buffer) {
            TokenType::Invalid
          } else if is_identifier(&buffer) {
            TokenType::Identifier
          } else {
            error::spawn(
              error::Error::Lexical, 
              format!("'{}' is invalid.", buffer), 
              Some(
                gen_error(
                  file, 
                  String::from(buffer.clone()), 
                  (line, col)
                )
              )
            ).unwrap();
            TokenType::Undefined
          },
        });

        buffer.clear();

      }

      match c {
        '"' => {
          let mut string: String = String::new();
          let mut last: char = '\0';
          
          loop {
            let ch = bytes.getc();
            if ch == None { 
              error::spawn(
                error::Error::TryAgain, 
                "Your code is incomplete. Please check it and try again.".to_string(),
                None
              ).unwrap()
            }

            if ch.unwrap() == '"' && last != '\\' { break; }
            string.push(ch.unwrap());
            last = ch.unwrap();
          }

          tokens.push(Token {
            location: (line, col),
            content: string,
            type_: TokenType::LiteralQuoute
          })
        }
        '\'' => {
          let mut char_: String = String::new();
          let mut last: char = '\0';

          loop {
            let ch = bytes.getc();
            if ch == None { 
              error::spawn(
                error::Error::TryAgain, 
                "Your code is incomplete. Please check it and try again.".to_string(),
                None
              ).unwrap()
            }

            if ch.unwrap() == '\'' && last != '\\' { break; }
            char_.push(ch.unwrap());
            last = ch.unwrap();
          }

          tokens.push(Token {
            location: (line, col),
            content: char_,
            type_: TokenType::LiteralChar
          })
        },
        _ => {
          if !c.is_whitespace() {
            if is_operator(c) {
              if is_double_operator(c) {

                let c2: Option<char> = bytes.getc();
                if c2 == None {
                  error::spawn(
                    error::Error::TryAgain, 
                    "Your code is incomplete. Please check it and try again.".to_string(),
                    None
                  ).unwrap()
                } 
                else if is_operator(c2.unwrap()) {
                  let x: usize = tokens.len();
                  let location = (line, col);
                  match (c, c2.unwrap()) {
                    ('>', '>') => tokens.push(Token {
                      content: String::from(">>"),
                      type_: TokenType::BinaryOperator("rShift"),
                      location
                    }),
                    ('<', '<') => tokens.push(Token {
                      content: String::from("<<"),
                      type_: TokenType::BinaryOperator("lShift"),
                      location
                    }),
                    ('=', '=') => tokens.push(Token {
                      content: String::from("=="),
                      type_: TokenType::Operator("eq"),
                      location
                    }),
                    ('=', '>') => tokens.push(Token {
                      content: String::from("=>"),
                      type_: TokenType::Operator("arrow"),
                      location
                    }),
                    ('-', '>') => tokens.push(Token {
                      content: String::from("->"),
                      type_: TokenType::Operator("implies"),
                      location
                    }),
                    ('!', '=') => tokens.push(Token {
                      content: String::from("!="),
                      type_: TokenType::Operator("ne"),
                      location
                    }),
                    (':', '=') => tokens.push(Token {
                      content: String::from(":="),
                      type_: TokenType::Operator("walrus"),
                      location
                    }),
                    ('>', '=') => tokens.push(Token {
                      content: String::from(">="),
                      type_: TokenType::Operator("ae"),
                      location
                    }),
                    ('<', '=') => tokens.push(Token {
                      content: String::from("<="),
                      type_: TokenType::Operator("le"),
                      location
                    }),
                    ('+', '+') => tokens.push(Token {
                      content: String::from("++"),
                      type_: TokenType::Operator("inc"),
                      location
                    }),
                    ('-', '-') => tokens.push(Token {
                      content: String::from("--"),
                      type_: TokenType::Operator("dec"),
                      location
                    }),
                    ('*', '=') => tokens.push(Token {
                      content: String::from("*="),
                      type_: TokenType::Operator("imul"),
                      location
                    }),
                    ('/', '=') => tokens.push(Token {
                      content: String::from("/="),
                      type_: TokenType::Operator("idiv"),
                      location
                    }),
                    ('&', '&') => tokens.push(Token {
                      content: String::from("&&"),
                      type_: TokenType::Operator("and"),
                      location
                    }),
                    ('|', '|') => tokens.push(Token {
                      content: String::from("||"),
                      type_: TokenType::Operator("or"),
                      location
                    }),
                    ('{', '*') if bytes.after() != None && bytes.after().unwrap() == '}' => {
                      bytes.sum();
                      tokens.push(Token {
                        content: String::from("{*}"),
                        type_: TokenType::Operator("all"),
                        location
                      })
                    },
                    _ => {
                      bytes.ungetc();
                    }
                  }
                  if x < tokens.len() { continue }
                }
              }

              tokens.push(Token {
                location: (line, col),
                content: String::from(c),
                type_: match c {
                  '|' | '&' | '~' | '^' => {
                    TokenType::BinaryOperator(
                      match c {
                        '|' => "or",
                        '&' => "and",
                        '^' => "xor",
                        '~' => "not",
                        _ => ""
                      }
                    )
                  }

                  '(' | ')' | '{' | '}' | '[' | ']' => {
                    match c {
                      '(' => TokenType::Parenthesis(true),
                      '{' => TokenType::Brace(true),
                      '[' => TokenType::Bracket(true),
                      ')' => TokenType::Parenthesis(false),
                      '}' => TokenType::Brace(false),
                      _ => TokenType::Bracket(false)
                    }
                  }

                  _ => {
                    TokenType::Operator(
                      match c {
                        '*' => "mul",
                        '/' => "div",
                        '-' => "sub",
                        '+' => "sum",
                        '=' => "sign",
                        '%' => "rest",
                        '!' => "not",
                        '$' => "pointer",
                        ':' => "colon",
                        ';' => "semi",
                        '.' => "dot",
                        ',' => "comma",

                        _ => {
                          error::spawn(error::Error::Lexical, format!("'{}' is invalid.", c), Some(gen_error(file, String::from(c), (line, col)))).unwrap();
                          ""
                        }
                      }
                    )
                  }
                }
              })
            } 
            
          }
        }
      }
      continue;
    } else { buffer.push(c); }
  }

  // println!("{:#?}", tokens);

  tokens
}