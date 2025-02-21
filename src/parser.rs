use crate::lexer::{Token, Type};
use std::fmt;

#[derive(Clone)]
pub enum TokenTree {
    Leaf(String),
    Node(Type, Box<TokenTree>, Box<TokenTree>) // mid left right
}

impl fmt::Display for TokenTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenTree::Leaf(value) => write!(f, "{}", value),
            TokenTree::Node(mid, left, right) => write!(f, "[ {} - {:#?} - {} ]", left, mid, right),
        }
    }
}

impl TokenTree {  
    pub fn parse_from_lexer(lexer_list: &Vec<Token>) -> Result<Self, &'static str> {
        let rpn = shunting_yard(&lexer_list);
        let mut stack: Vec<TokenTree> = Vec::new();

        for t in rpn {
            match t.tag {
                Type::VALUE | Type::VAR => stack.push(TokenTree::Leaf(t.word)),
                _ => {
                    let right = stack.pop(); // needs to be inverted
                    let left = stack.pop();
                    let node = TokenTree::Node(t.tag, Box::new(left.unwrap()), Box::new(right.unwrap()));
                    stack.push(node);
                }, 
            }
        }

        if stack.len() != 1 {
            Err("failed to parse into tree")
        }else { Ok(stack.last().unwrap().clone()) } 
    }
    
    // error if x is in the equation
    pub fn calculate(&self) -> f32{
        match self {
            TokenTree::Leaf(value) => value.parse::<f32>().unwrap(),
            TokenTree::Node(m, l, r) => {
                match m {
                    Type::PLUS => l.calculate() + r.calculate(),
                    Type::MINUS => l.calculate() - r.calculate(),
                    Type::TIMES => l.calculate() * r.calculate(),
                    Type::FRAC => l.calculate() / r.calculate(), // obv error if div through 0
                    _ => 0.0,
                }
            }
        }
    }
}

fn shunting_yard(input: &Vec<Token>) -> Vec<Token> {
    let mut queue: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();
    
    for t in input {
        match t.tag {
            Type::VALUE | Type::VAR => queue.push(t.clone()),
            Type::PLUS | Type::MINUS | Type::TIMES | Type::FRAC => {
                while let Some(s) = stack.last() {
                    if s.tag.get_precedence() >= t.tag.get_precedence() {
                        queue.push(s.clone());
                        stack.pop();
                    }else {break;}
                } 
                stack.push(t.clone());
            },
            Type::BLEFT => stack.push(t.clone()),
            Type::BRIGHT => {
                while stack.last().expect("no value").tag != Type::BLEFT {
                    queue.push(stack.last().expect("no value").clone());
                    stack.pop();
                }
                stack.pop(); // pop left bracket
            }
        }
    }
    stack.reverse();
    queue.append(&mut stack);
    
    println!("{:?}", queue);

    queue
}
