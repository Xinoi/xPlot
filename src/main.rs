mod input;
mod lexer;
mod parser;

fn main() {
    //print read list
    let final_input = combine_numbers_and_chars(input::get_input().expect("function not correct"));
    println!("{:?}", final_input);

    //print lexed list 
    let input_lexed = lexer::tag(&final_input);
    lexer::print_tokens(&input_lexed);
}

fn combine_numbers_and_chars(input: Vec<char>) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_number = String::new();
    //combine all numbers next to each other to a single one
    for ch in input {
        if ch.is_ascii_digit() {
            current_number.push(ch);
        } else {
            if !current_number.is_empty() {
                result.push(current_number.clone());
                current_number.clear();
            }
            result.push(ch.to_string());
        }
    }
    if !current_number.is_empty() {
        result.push(current_number);
    }
    result
}
