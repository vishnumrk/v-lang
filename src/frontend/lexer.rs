use std::collections::HashMap;
use std::process::exit;

use crate::frontend::token::Token;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, Token> = {
        let mut map = HashMap::new();
        map.insert("let", Token::Let);
        map.insert("const", Token::Const);
        map
    };
}
pub fn tokenize(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    if !source.is_ascii() {
        panic!("language does not support non ancii characters in source code");
    }
    let source_chars: &[u8] = source.as_bytes();
    let mut index = 0;
    while index < source_chars.len() {
        let char_at_index = source_chars[index] as char;
        match char_at_index {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '{' => tokens.push(Token::OpenBrace),
            '}' => tokens.push(Token::CloseBrace),
            '[' => tokens.push(Token::OpenBracket),
            ']' => tokens.push(Token::CloseBracket),
            '-' | '+' | '*' | '/' | '%' => {
                tokens.push(Token::BinaryOperator(char_at_index.to_owned()))
            }
            '=' => tokens.push(Token::Equals),
            ';' => tokens.push(Token::Semicolon),
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),
            '.' => tokens.push(Token::Dot),
            ' ' | '\n' | '\t' | '\r' => {}
            _ => {
                if char_at_index.is_ascii_digit() {
                    let mut num_str = String::new();
                    while index < source_chars.len() {
                        let next_char = source_chars[index] as char;
                        if next_char.is_ascii_digit() {
                            num_str.push(next_char);
                            index += 1;
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(num_str.parse::<isize>().unwrap()));
                    index -= 1;
                } else if char_at_index.is_alphabetic() {
                    let mut str = String::new();
                    while index < source_chars.len() {
                        let next_char = source_chars[index] as char;
                        if next_char.is_alphabetic() {
                            str.push(next_char);
                            index += 1;
                        } else {
                            break;
                        }
                    }
                    if KEYWORDS.contains_key(str.as_str()) {
                        tokens.push(KEYWORDS.get(str.as_str()).unwrap().clone());
                    } else {
                        tokens.push(Token::Identifier(str));
                    }

                    index -= 1;
                } else {
                    eprintln!("'{}' invalid character found", char_at_index);
                    exit(2);
                }
            }
        }
        index += 1;
    }
    tokens.push(Token::Eof);
    tokens
}
