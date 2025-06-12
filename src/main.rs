use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use std::iter::Peekable;

#[derive(Debug, Clone)]
enum Token {
    Var(char),
    Not,     // !
    And,     // &
    Or,      // |
    Xor,     // ^
    Implies, // ->
    Iff,     // <->
    LParen,  // (
    RParen,  // )
}

fn tokenize(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();
    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' => {
                chars.next();
            }
            '!' => {
                tokens.push(Token::Not);
                chars.next();
            }
            '&' => {
                tokens.push(Token::And);
                chars.next();
            }
            '|' => {
                tokens.push(Token::Or);
                chars.next();
            }
            '^' => {
                tokens.push(Token::Xor);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '-' => {
                chars.next();
                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(Token::Implies);
                } else {
                    panic!("Invalid token: expected ->");
                }
            }
            '<' => {
                chars.next();
                if chars.next() == Some('-') && chars.next() == Some('>') {
                    tokens.push(Token::Iff);
                } else {
                    panic!("Invalid token: expected <->");
                }
            }
            c if c.is_ascii_alphabetic() => {
                tokens.push(Token::Var(c));
                chars.next();
            }
            _ => panic!("Unexpected character: {}", ch),
        }
    }
    tokens
}

struct Parser<'a> {
    tokens: Peekable<std::slice::Iter<'a, Token>>,
    vars: &'a HashMap<char, bool>,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token], vars: &'a HashMap<char, bool>) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
            vars,
        }
    }

    fn parse_expr(&mut self) -> bool {
        self.parse_iff()
    }

    fn parse_iff(&mut self) -> bool {
        let mut result = self.parse_implies();
        while let Some(Token::Iff) = self.tokens.peek() {
            self.tokens.next();
            let rhs = self.parse_implies();
            result = result == rhs;
        }
        result
    }

    fn parse_implies(&mut self) -> bool {
        let mut result = self.parse_or();
        while let Some(Token::Implies) = self.tokens.peek() {
            self.tokens.next();
            let rhs = self.parse_or();
            result = !result || rhs;
        }
        result
    }

    fn parse_or(&mut self) -> bool {
        let mut result = self.parse_xor();
        while let Some(Token::Or) = self.tokens.peek() {
            self.tokens.next();
            result |= self.parse_xor();
        }
        result
    }

    fn parse_xor(&mut self) -> bool {
        let mut result = self.parse_and();
        while let Some(Token::Xor) = self.tokens.peek() {
            self.tokens.next();
            result ^= self.parse_and();
        }
        result
    }

    fn parse_and(&mut self) -> bool {
        let mut result = self.parse_not();
        while let Some(Token::And) = self.tokens.peek() {
            self.tokens.next();
            result &= self.parse_not();
        }
        result
    }

    fn parse_not(&mut self) -> bool {
        if let Some(Token::Not) = self.tokens.peek() {
            self.tokens.next();
            !self.parse_not()
        } else {
            self.parse_atom()
        }
    }

    fn parse_atom(&mut self) -> bool {
        match self.tokens.next() {
            Some(Token::Var(c)) => *self.vars.get(c).unwrap_or(&false),
            Some(Token::LParen) => {
                let val = self.parse_expr();
                match self.tokens.next() {
                    Some(Token::RParen) => {}
                    _ => panic!("Expected ')'"),
                }
                val
            }
            Some(tok) => panic!("Unexpected token: {:?}", tok),
            None => panic!("Unexpected end of input"),
        }
    }
}

fn extract_variables(expr: &str) -> Vec<char> {
    let vars: HashSet<char> = expr.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    let mut vars_vec: Vec<char> = vars.into_iter().collect();
    vars_vec.sort();
    vars_vec
}

fn main() {
    print!("Please enter a boolean expression (support ! & | ^ -> <->): \n");
    io::stdout().flush().unwrap();

    let mut expr = String::new();
    io::stdin().read_line(&mut expr).unwrap();
    let expr = expr.trim();

    // let expr = "((A & B) -> (!C | D)) <-> (!A | (C & (D -> B)))"; // test expression

    let tokens = tokenize(&expr);
    let variables = extract_variables(&expr);

    if variables.len() > 5 {
        println!("Error: More than 5 variables are not supported.");
        return;
    }

    for var in &variables {
        print!("{} ", var);
    }
    println!("| Result");

    let total = 1 << variables.len();

    for i in 0..total {
        let mut var_map = HashMap::new();
        for (j, var) in variables.iter().enumerate() {
            let val = (i >> (variables.len() - j - 1)) & 1 == 1;
            var_map.insert(*var, val);
            print!("{} ", if val { 1 } else { 0 });
        }
        let mut parser = Parser::new(&tokens, &var_map);
        let result = parser.parse_expr();
        println!("| {}", if result { 1 } else { 0 });
    }
}
