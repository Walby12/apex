use std::collections::HashMap;
use std::process;
use std::fs;

#[derive(Debug, Clone)]
enum TokenType {
    Number,
    Identifier,
    Equals,
    Let,
    OpenParen,
    CloseParen,
    BinaryOperator,
}

#[derive(Debug)]
struct Token {
    value: String,
    type_: TokenType,
}

fn token(value: String, type_: TokenType) -> Token {
    Token {
        value,
        type_,
    }
} 

fn tokenize(source_code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut src: Vec<char> = source_code.chars().collect();

    while src.len() > 0 {
        if src[0] == '(' {
            tokens.push(token(String::from(src.remove(0)), TokenType::OpenParen));
        } else if src[0] == ')' {
            tokens.push(token(String::from(src.remove(0)), TokenType::CloseParen));
        } else if src[0] == '+' || src[0] == '-' || src[0] == '*' || src[0] == '/' {
            tokens.push(token(String::from(src.remove(0)), TokenType::BinaryOperator));
        } else if src[0] == '=' {
            tokens.push(token(String::from(src.remove(0)), TokenType::Equals));
        } else {
            if src[0].is_numeric() {
                let mut num = String::new();

                while src.len() > 0 && src[0].is_numeric() {
                    num.push(src.remove(0));
                }

                tokens.push(token(num, TokenType::Number));
            } else if src[0].is_alphabetic() {
                let mut ident = String::new();

                while src.len() > 0 && src[0].is_alphabetic() {
                    ident.push(src.remove(0));
                }

                let mut reserved_keywords: HashMap<&str, TokenType> = HashMap::new();
                reserved_keywords.insert("let", TokenType::Let);

                if let Some(token_type) = reserved_keywords.get(ident.as_str()) {
                    tokens.push(token(ident, token_type.clone()));
                } else {
                    tokens.push(token(ident, TokenType::Identifier));
                }
                
            } else if src[0].is_whitespace() {
                src.remove(0);
            } else {
                println!("Unrecognized char found in src: {:?}", src[0]);
                process::exit(1);
            }
        } 

    }
    return tokens;
}

fn main() {
    let source = fs::read_to_string("file.txt").unwrap();

    for token in tokenize(source) {
        println!("{:?}", token);
    }
}
