mod input;
mod lexer;
mod parser;

use macroquad::prelude::*;
use miniquad::window::screen_size;

struct Vars {
    mid: (f32, f32),
    scale: f32, 
    step: f32, 
    font_size: f32, 
    iterations: i32,
}

impl Vars {
    fn new(scale: f32) -> Self {
        Self {
            mid: (screen_width() / 2.0, screen_height() / 2.0),
            scale, 
            step: 50.0 * scale,
            font_size: 7.0 * scale,
            iterations: 20,
        }
    }

    fn update(&mut self, scale: f32) {
        self.mid = (screen_width() / 2.0, screen_height() / 2.0);
        self.scale = scale;
        self.step = 50.0 * scale;
        self.font_size = 7.0 * scale;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
   
    let mut vars = Vars::new(1.0);

    //print read list
    let final_input = combine_numbers_and_chars(input::get_input().expect("function not correct"));
    println!("{:?}", final_input);

    //print lexed list 
    let input_lexed = lexer::tag(&final_input);
    lexer::print_tokens(&input_lexed);

    let tree = parser::TokenTree::parse_from_lexer(&input_lexed).unwrap(); 
    println!("{}", &tree);

    let points = tree.get_points(-10..10, vars.iterations);
    println!("points: {:?}", points);
    

    

    draw_graph(&mut vars, &points).await;
}

async fn draw_graph(vars: &mut Vars, points: &Vec<(f32, f32)>) {
    let mut current_scale = 1.0;

    let mut cam = Camera2D::from_display_rect(Rect::new(0.0, screen_size().1, screen_size().0, -screen_size().1));

    loop {
        set_camera(&cam);

        for key in get_keys_pressed() {
           match key {
               KeyCode::PageUp => current_scale += 0.1,
               KeyCode::PageDown => current_scale -= 0.1, 
               _ => {}
           }
        } 

        vars.update(current_scale);
        let mut points_it = points.into_iter().peekable();
        clear_background(BLACK);

        grid(&vars);
        axis(&vars);

        while let Some(current) = points_it.next() {
            if let Some(next) = points_it.peek() {
                let (x1, y1) = calc_cords(&vars, current);
                let (x2, y2) = calc_cords(&vars, next);
                draw_line(x1, y1, x2, y2, 1.2, BLUE);
            }
        } 
        
        next_frame().await;
    }
}

fn axis(vars: &Vars) {

    draw_line(0.0, screen_height() / 2.0, screen_width(), screen_height() / 2.0, 1.0, WHITE); 
    draw_line(screen_width() / 2.0, 0.0, screen_width() / 2.0, screen_height(), 1.0, WHITE);
    
    // x lines
    for i in 1..=((screen_width() / 2.0) / vars.step) as i32 {
        draw_line(vars.mid.0 + (i as f32 * vars.step), vars.mid.1 + 7.0, vars.mid.0 + (i as f32 * vars.step), vars.mid.1-7.0, 1.0, WHITE);
        draw_line(vars.mid.0 + (-i as f32 * vars.step), vars.mid.1 + 7.0, vars.mid.0 + (-i as f32 * vars.step), vars.mid.1-7.0, 1.0, WHITE);
        if i != 0 {
            draw_text(&i.to_string(), vars.mid.0 + (i as f32 * vars.step) - vars.font_size / 2.0, vars.mid.1 - vars.font_size - 2.0, 18.0 * vars.scale, WHITE);
            draw_text(&(-i).to_string(), vars.mid.0 + (-i as f32 * vars.step) - vars.font_size, vars.mid.1 - vars.font_size - 2.0, 18.0 * vars.scale, WHITE);
        }
    }   
    // y lines
    for i in 1..=((screen_height() / 2.0) / vars.step) as i32 {
        draw_line(vars.mid.0 - 7.0, vars.mid.1 + (i as f32 * vars.step), vars.mid.0 + 7.0, vars.mid.1 + (i as f32 * vars.step), 1.0, WHITE);
        draw_line(vars.mid.0 - 7.0, vars.mid.1 + (-i as f32 * vars.step), vars.mid.0 + 7.0, vars.mid.1 + (-i as f32 * vars.step), 1.0, WHITE);
        if i != 0 {
            draw_text(&(-i).to_string(), vars.mid.0 + vars.font_size + 2.0, vars.mid.1 + (i as f32 * vars.step) + vars.font_size / 2.0, 18.0 * vars.scale, WHITE);
            draw_text(&i.to_string(), vars.mid.0 + vars.font_size + 2.0, vars.mid.1 + (-i as f32 * vars.step) + vars.font_size / 2.0, 18.0 * vars.scale, WHITE);
        }
    }
}

fn grid(vars: &Vars) {
    let grey = Color::new(163.0, 163.0, 163.0, 0.05);
    // x axis lines 
    for i in 1..=(screen_height() / vars.step) as i32 {
        draw_line(0.0, i as f32 * vars.step, screen_width(), i as f32 * vars.step, 1.0, grey);
    }

    // y axis lines 
    for i in 1..=(screen_width() / vars.step) as i32 {
        draw_line(i as f32 * vars.step, 0.0, i as f32 * vars.step, screen_height(), 1.0, grey);
    }
}

fn calc_cords(vars: &Vars, point: &(f32, f32)) -> (f32, f32) {
    let x = vars.mid.0 + (point.0 * vars.step * vars.scale);
    let y = vars.mid.1 - (point.1 * vars.step * vars.scale);
    (x, y)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Graph".to_owned(),
        window_width: 1200,
        window_height: 900,
        sample_count: 16,
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
