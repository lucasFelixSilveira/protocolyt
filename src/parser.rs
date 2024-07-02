pub mod utils;
use crate::parser::utils::Tokens;

use std::path;

use crate::error;
use crate::lexer;
use lexer::utils::Bytes;
use lexer::TokenType;
use lexer::Token;

use colored::*;

#[derive(Debug, Clone, PartialEq)]
enum BracketType {
  Parenthesis,
  Brace,
  Bracket,
  None
}

fn gen_error(file: &mut (Bytes, String, String), content: String, location: (usize, usize), msg: String) -> String {
  let (_, name) =  file.2.rsplit_once(path::MAIN_SEPARATOR).unwrap();
  let lines: Vec<&str> = file.1.lines().collect();
  format!(
    "In {name} {}:{} ocurred a syntax error:\n  {} {}\n  {}{}{} {}", 
    location.0, location.1, format!("{} |", location.0).blue(),
    lines[location.0 - 1],
    " ".repeat(format!("{} |", location.0).len() + location.1), "^".repeat(content.len()).green(), "~".green(), msg.red()
  )
}

fn incomplete() {
  error::spawn(
    error::Error::TryAgain, 
    "Your code is incomplete. Please check it and try again.".to_string(),
    None
  ).unwrap()
}

fn has_expected(file: &mut (Bytes, String, String), tk: Token, expected_type: TokenType) {
  error::spawn(
    error::Error::InvalidOperation,
    "Another value was expected.".to_string(),
    Some(gen_error(file, tk.content, tk.location, format!("Has {} was expected.", format!("{:?}", expected_type).to_lowercase())))
  ).unwrap();
}

pub fn ts_generic(generic: String) -> String {
  let g0: String = if generic.starts_with('@') { ["$".to_string(), generic[1..].to_string()].concat() } else {generic};
  let g1: Vec<&str> = g0.split('[').collect();
  let g2: String = g1.join("<");
  let g3: Vec<&str> = g2.split(']').collect();
  g3.join(">")
}

pub fn parse(file_content: &mut (Bytes, String, String), t: Vec<Token>) -> String {
  let std_prefix: String = r#"
type $void = void;
function $error(msg) { console.log(msg) };
function $import(path) { if( path.endsWith(".ply") ) return require(path) 
else if( ["vector", "asciz", "io", "types"].includes(path) ) { return require("../std/"+path+".ts") } };
try { main(require('process').argv.slice(2)) } catch { console.log("Segmentation fault"); }
require("process").exit(0);"#.to_string();

  let mut tokens: Tokens = Tokens::from(t);
  let mut result: String = String::new();
  let mut stack: Vec<BracketType> = Vec::new();
  let mut expressions: Vec<Vec<Token>> = Vec::new();
  let mut expression: Vec<Token> = Vec::new();

  loop {
    let first: Option<Token> = tokens.next();
    if first == None { break; }
    
    let tk: Token = first.unwrap();
    match tk.type_ {
      TokenType::Keyword => {
        match tk.content.as_str() {
          "await" => { result.push_str(" await ") }
          "const" => {
            let id: Token = tokens.next().unwrap_or_else(|| { incomplete(); unreachable!() });
            if 
              id.type_ != TokenType::Identifier && 
              id.type_ != TokenType::Operator("all") 
            { has_expected(file_content, id.clone(), TokenType::Identifier) }

            result.push_str("\nconst ");
            result.push_str(&id.content);

            let collon_or_walrus: Token = tokens.next().unwrap_or_else(|| { incomplete(); unreachable!() });
            if collon_or_walrus.type_ != TokenType::Operator("walrus") { has_expected(file_content, id.clone(), TokenType::Operator("walrus")) }
          
            if collon_or_walrus.type_ == TokenType::Operator("walrus") {
              result.push_str(": any = ");
            }
          }
          "type" => {
            let id: Token = tokens.next().unwrap_or_else(|| { incomplete(); unreachable!() });
            if id.type_ != TokenType::Identifier { has_expected(file_content, id.clone(), TokenType::Identifier) }

            result.push_str("\ntype ");
            result.push_str(&id.content);

            let sign: Token = tokens.next().unwrap_or_else(|| { incomplete(); unreachable!() });
            if sign.type_ != TokenType::Operator("sign") { has_expected(file_content, id.clone(), TokenType::Operator("sign")) }
            result.push_str(" = ");
          }
          "as" => {
            result.push_str(" as ");
            let next: Token = tokens.next().unwrap_or_else(|| { incomplete(); unreachable!() });
            result.push_str(&next.content);
          }
          "fn" => {
            let id: Token = tokens.next().unwrap_or_else(|| { incomplete(); unreachable!() });
            if id.type_ != TokenType::Identifier { has_expected(file_content, id.clone(), TokenType::Identifier) }

            if id.content == "main" { result.push_str(&std_prefix); }

            result.push_str(&format!("\nasync function {}", if id.content.starts_with('@') { ["$", id.content[1..].to_string().as_str()].concat() } else { id.content }));
            let paren: Token = tokens.next().unwrap_or_else(|| { incomplete(); unreachable!() });
            if 
              paren.type_ != TokenType::Parenthesis(true) &&
              paren.type_ != TokenType::Bracket(true) 
            { has_expected(file_content, paren.clone(), TokenType::Parenthesis(true)) }
          
            let mut generics: Vec<String> = Vec::new();
            if paren.type_ == TokenType::Bracket(true) {
              loop {
                let r#type: Token = tokens.next().unwrap_or_else(|| { incomplete(); unreachable!() });
                if r#type.type_ != TokenType::Identifier { has_expected(file_content, r#type.clone(), TokenType::Identifier) }
                generics.push(r#type.content);

                let final_or_comma: Token = tokens.next().unwrap_or_else(|| { incomplete(); unreachable!() });
                if final_or_comma.type_ == TokenType::Bracket(false) { break }
                if final_or_comma.type_ == TokenType::Operator("comma") { continue }
                else { has_expected(file_content, paren.clone(), TokenType::Bracket(false)) }
              }
            }

            if generics.len() > 0 {
              result.push('<');
              result.push_str(&generics.first().unwrap());
              if generics.len() > 1 {
                for generic in generics.clone().into_iter().skip(1) {
                  result.push_str(&[", ", generic.as_str()].concat());
                }
              }
              result.push('>');

              let open: Token = tokens.next().unwrap_or_else(|| { incomplete(); unreachable!() });
              if open.type_ != TokenType::Parenthesis(true) { has_expected(file_content, open.clone(), TokenType::Parenthesis(true)) }
            }

            
            let mut params: Vec<(String, String, String)> = Vec::new();
            loop {
              let identifier: Token = tokens.next().unwrap_or_else(|| { incomplete(); unreachable!() });
              if 
                identifier.type_ != TokenType::Identifier &&
                identifier.type_ != TokenType::Parenthesis(false)
              { has_expected(file_content, identifier.clone(), TokenType::Identifier) }
            
              let colon: Token = tokens.next().unwrap_or_else(|| { incomplete(); unreachable!() });
              if colon.type_ != TokenType::Operator("colon") { has_expected(file_content, colon.clone(), TokenType::Operator("colon")) }
            
              let mut need_break: bool = false;
              let mut string: String = String::new();
              let mut natural: String = String::new();
              loop {
                let tk: Token = tokens.next().unwrap_or_else(|| { incomplete(); unreachable!() });
                match tk.type_ {
                  TokenType::Bracket(false) if stack.last().unwrap() == &BracketType::Bracket => { string.push(']'); stack.remove(stack.len()-1); }, 
                  TokenType::Bracket(true) => { stack.push(BracketType::Bracket); string.push('['); continue },
                  TokenType::Identifier => { if natural.is_empty() { natural = tk.content.clone() }; string.push_str(&tk.content); continue },
                  TokenType::Operator("comma") if stack.last().unwrap_or_else(|| &BracketType::None) != &BracketType::Bracket => { break },
                  TokenType::Operator("comma") => { string.push_str(", "); continue },
                  TokenType::Parenthesis(false) => { need_break = true; break }
                  _ => { has_expected(file_content, paren.clone(), TokenType::Type) }
                }
              }

              params.push(( identifier.content, string, natural ));
              if need_break { break }
            }

            result.push_str("(...$arguments: any[]): ");

            let mut return_type: String = String::new();
            let mut natural: String = String::new();
            loop {
              let tk: Token = tokens.next().unwrap_or_else(|| { incomplete(); unreachable!() });
              match tk.type_ {
                TokenType::Bracket(false) if stack.last().unwrap() == &BracketType::Bracket => { return_type.push(']'); stack.remove(stack.len()-1); }, 
                TokenType::Bracket(true) => { stack.push(BracketType::Bracket); return_type.push('['); continue },
                TokenType::Identifier => { if natural.is_empty() { natural = tk.content.clone() }; return_type.push_str(&tk.content); continue },
                TokenType::Operator("comma") if stack.last().unwrap_or_else(|| &BracketType::None) == &BracketType::Bracket => { return_type.push_str(", "); continue },
                TokenType::Brace(true) => { break }
                _ => { has_expected(file_content, tk.clone(), TokenType::Type) }
              }
            }

            result.push_str(&ts_generic(return_type.clone()));
            result.push_str(" {");

            for (name, type_, type_natural) in params {
              if generics.contains(&type_) { continue }
              result.push_str(&format!("\ntry {{ if( {type_natural} == undefined ) $error('The \\'{type_natural}\\' type does not exist or is not imported.'); }} catch {{ $error('The \\'{type_natural}\\' type does not exist or is not imported.') }}"));
              result.push_str(&format!("\nconst {name}: {} = $arguments.shift();", ts_generic(type_)));
            }

            if return_type != "@void" {
              result.push_str(&format!("\ntry {{ if( {natural} == undefined ) $error('The \\'{natural}\\' type does not exist or is not imported.'); }} catch {{ $error('The \\'{natural}\\' type does not exist or is not imported.') }}"));
            }
            result.push('\x0a');

            stack.push(BracketType::Brace);

          }
          _ => {}
        }
      }
      TokenType::Brace(false) if stack.last().unwrap_or_else(|| &BracketType::None) == &BracketType::Brace => { result.push('}') }
      TokenType::Parenthesis(false) if stack.last().unwrap_or_else(|| &BracketType::None) == &BracketType::Parenthesis => { result.push(')'); stack.pop(); }
      TokenType::Parenthesis(true) => { result.push('('); stack.push(BracketType::Parenthesis) }
      TokenType::LiteralQuoute => result.push_str(&['"'.to_string(), tk.content.clone(), '"'.to_string()].concat()),
      TokenType::Identifier if tk.content.starts_with('@') => {
        result.push_str(&["$", tk.content[1..].to_string().as_str()].concat())
      }
      TokenType::Identifier => { result.push_str(&tk.content) }
      TokenType::Operator("semi") => { result.push_str(";"); }
      TokenType::Operator("dot") => { result.push('.') }
      _ => {}
    }
  }

  result.trim().to_string()
}