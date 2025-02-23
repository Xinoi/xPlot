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
    
    pub fn get_points(&self, def: std::ops::Range<i32>, steps: i32) -> Vec<(f32, f32)> {
        let mut result: Vec<(f32, f32)> = Vec::new();
        
        for i in def {
            for step in 0..steps {
                let x = i as f32 + (step as f32 / steps as f32);
                let y = self.fill_x(x).calculate();
                result.push((x, y));
            }
        }
        result
    }

    fn fill_x(&self, x: f32) -> Self {
        match self {
            TokenTree::Node(m, l, r) => TokenTree::Node(m.clone(), Box::new(l.fill_x(x)), Box::new(r.fill_x(x))), 
            TokenTree::Leaf(value) => {
                if value == "x" {
                    TokenTree::Leaf(x.to_string())
                }else { TokenTree::Leaf(value.clone()) } 
        }
    }
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
                    Type::POW => l.calculate().powf(r.calculate()),
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
            Type::PLUS | Type::MINUS | Type::TIMES | Type::FRAC | Type::POW => {
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
