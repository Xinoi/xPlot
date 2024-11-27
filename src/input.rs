use std::io::{stdin, stdout, Write};

pub fn get_input() -> Result<Vec<char>, String> {
   let i_str = get_input_string(); 
   let i_vec = get_input_vec(&i_str);
   match check_input(&i_vec) {
       Ok(_) => Ok(i_vec.clone()),
       Err(e) => Err(e),
   }
}

fn get_input_string() -> String {
    print!("f(x) = ");
    stdout().flush().expect("Could not flush output!");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("error: input incorrect String");
    input
}

fn get_input_vec(input: &String) -> Vec<char> {
    let mut elements: Vec<char> = Vec::new();
    for c in input.trim().chars() {
         elements.push(c);
    }
    elements
}

fn check_input(input_v: &Vec<char>) -> Result<(), String> {
    let valid_symbols: [char; 6] = ['+', '-', '*', '/', '(', ')']; 
    let mut input_v_iter = input_v.iter().peekable();
    while let Some(symbol) = input_v_iter.next() { 
        // check for unvalid chars 
        if !(valid_symbols.contains(symbol) || symbol.is_numeric() || *symbol == 'x') {
            return Err("first check error: The Function needs to match the Criteria!".to_string());
        }
        //check if two symbols get repeated
        if let Some(next_symbol) = input_v_iter.peek() {
            if symbol == *next_symbol {
                if *symbol == '(' || *symbol == ')' || symbol.is_numeric() {
                    continue;
                }else {
                    return Err("first check error: Something in your function looks wrong!".to_string());
                }
            }
        }
    }
    Ok(()) 
}

