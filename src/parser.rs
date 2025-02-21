use crate::lexer::{Token, Type};

struct TokenTree {
    mid: Option<Token>, 
    left: Box<Option<TokenTree>>, 
    right: Box<Option<TokenTree>>, 
}
impl TokenTree {
    fn head() -> Self {
        Self {
            mid: None, 
            left: Box::new(None),
            right: Box::new(None),
        }
    }
    fn parse_from_lexer(&self, lexer_list: &Vec<Token>) {
         
    }
}

// todo! remove pub
pub fn shunting_yard(input: &Vec<Token>) -> Vec<Token> {
    let mut queue: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();
    
    for t in input {
        match t.tag {
            Type::VALUE | Type::VAR => queue.push(t.clone()),
            Type::PLUS | Type::MINUS | Type::TIMES | Type::FRAC => {
                println!("got operator!");
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
                println!("got right b");
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

    println!("{:#?}", queue);

    queue

}
