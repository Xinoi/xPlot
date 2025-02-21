mod input;
mod lexer;
mod parser;

use macroquad::prelude::*;

static W_HEIGHT: f32 = 800.0;
static W_WIDTH: f32 = 1200.0;
static MID: (f32, f32) = (W_WIDTH / 2.0, W_HEIGHT / 2.0);
static SCALE: f32 = 1.0;
static STEP: f32 = 20.0 * SCALE;

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
    draw_line(0.0, W_HEIGHT / 2.0, W_WIDTH, W_HEIGHT / 2.0, 1.0, WHITE); 
    draw_line(W_WIDTH / 2.0, 0.0, W_WIDTH / 2.0, W_HEIGHT, 1.0, WHITE);
    
    // x lines
    for i in 0..((W_WIDTH / 2.0) / STEP) as i32 {
        draw_line(MID.0 + (i as f32 * STEP), MID.1 + 5.0, MID.0 + (i as f32 * STEP), MID.1-5.0, 1.0, WHITE);
        draw_line(MID.0 + (-i as f32 * STEP), MID.1 + 5.0, MID.0 + (-i as f32 * STEP), MID.1-5.0, 1.0, WHITE);
    }   
    // y lines
    for i in 0..((W_HEIGHT / 2.0) / STEP) as i32 {
        draw_line(MID.0 - 5.0, MID.1 + (i as f32 * STEP), MID.0 + 5.0, MID.1 + (i as f32 * STEP), 1.0, WHITE);
        draw_line(MID.0 - 5.0, MID.1 + (-i as f32 * STEP), MID.0 + 5.0, MID.1 + (-i as f32 * STEP), 1.0, WHITE);

    }

}

fn window_conf() -> Conf {
    Conf {
        window_title: "Graph".to_owned(),
        window_width: W_WIDTH as i32,
        window_height: W_HEIGHT as i32,
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
