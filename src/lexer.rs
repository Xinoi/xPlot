// lexing the input
// the output should be something like that:
// { ("23", INT), ("+", PLUS), ("x", VARIABLE) }
use std::fmt;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Type {
    PLUS,
    MINUS,
    TIMES,
    FRAC,
    BLEFT,
    BRIGHT,
    VAR,
    VALUE,
}

impl Type {
    pub fn get_precedence(&self) -> i8{
        match self {
            Self::PLUS => 2,
            Self::MINUS => 2,
            Self::TIMES => 3,
            Self::FRAC => 3,
            _ => -1
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Token { 
    pub word: String,
    pub tag: Type,
}

impl Token {
    fn new(w: String, t: Type) -> Token {
        Token {
            word: w,
            tag: t,
        }
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "('{}', {:#?})", self.word, self.tag)
    }
}

pub fn tag(input: &Vec<String>) -> Vec<Token> { 
    let mut tags: Vec<Token> = Vec::new();
    
    for w in 0..input.len() {
        match input.get(w).expect("no element").chars().next().unwrap() {
            '+' => tags.push(Token::new("+".to_string(), Type::PLUS)),
            '-' => tags.push(Token::new("-".to_string(), Type::MINUS)),
            '*' => tags.push(Token::new("*".to_string(), Type::TIMES)),
            '/' => tags.push(Token::new("/".to_string(), Type::FRAC)),
            '(' => tags.push(Token::new("(".to_string(), Type::BLEFT)),
            ')' => tags.push(Token::new(")".to_string(), Type::BRIGHT)),
            'x' => {
                let pre = if w > 0 {input.get(w-1)}else { None };
                let post = input.get(w+1);

                if let Some(e) = pre {
                    if e.chars().next().unwrap().is_digit(10) || e.chars().next().unwrap() == ')' {
                        tags.push(Token::new("*".to_string(), Type::TIMES));
                        tags.push(Token::new("x".to_string(), Type::VAR));
                        continue;
                    }
                }
                if let Some(e) = post {
                    if e.chars().next().unwrap().is_digit(10) || e.chars().next().unwrap() == '(' {
                        tags.push(Token::new("x".to_string(), Type::VAR));
                        tags.push(Token::new("*".to_string(), Type::TIMES));
                        continue;
                    } 
                }
                tags.push(Token::new("x".to_string(), Type::VAR));
            },
            _ => tags.push(Token::new(input.get(w).expect("no element").to_string(), Type::VALUE)),
        }
    }
    tags
}

pub fn print_tokens(token_list: &Vec<Token>) {
    print!("[ ");
    for t in token_list {
        if t == token_list.last().expect("Tokenlist empty")  {
            print!("{}", t);
        }else {
            print!("{}, ", t);
        }
    }
    println!(" ]");
}



