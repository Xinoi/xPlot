mod input;

fn main() {

    println!("-- input function (only +, -, /, * and Brackets are supported) --\n-- Use x as the variable! --");
    while let Err(error) = input::get_input() {
        println!("{}", error);
    } 
}