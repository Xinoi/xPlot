mod input;
mod lexer;
mod parser;

use macroquad::prelude::*;

static W_HEIGHT: i32 = 800;
static W_WIDTH: i32 = 1200;

#[macroquad::main(window_conf)]
async fn main() {
    //print read list
    let final_input = combine_numbers_and_chars(input::get_input().expect("function not correct"));
    println!("{:?}", final_input);

    //print lexed list 
    let input_lexed = lexer::tag(&final_input);
    lexer::print_tokens(&input_lexed);

    let tree = parser::TokenTree::parse_from_lexer(&input_lexed).unwrap(); 
    println!("{}", &tree);
    

    draw_graph().await;

}

async fn draw_graph() {
    loop {
        clear_background(BLACK);

        axis();

        next_frame().await;
    }
}

fn axis() {
   draw_line(0.0, (W_HEIGHT / 2) as f32, W_WIDTH as f32, (W_HEIGHT / 2) as f32, 100.0, WHITE); 
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Graph".to_owned(),
        window_width: W_WIDTH,
        window_height: W_HEIGHT,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandWithX11Fallback,
            ..Default::default()
        }, 
        ..Default::default()
    }

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
