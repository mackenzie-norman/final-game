use console_engine::pixel::{self, Pixel};
use console_engine::rect_style::BorderStyle;
use console_engine::{screen, Color, MouseButton};
use console_engine::ConsoleEngine;
use console_engine::KeyCode;
use figlet_rs::FIGfont; 
use rand::{random, Rng, rngs::StdRng, SeedableRng};
mod dialouge;
use dialouge::{pt_in_box, tutorial_skipping, Dialouge};
mod character;
use character::Character;
mod pov;
use pov::{close_eyes, open_eyes, waking_up};
mod smart_drawing;
use smart_drawing::{line, fill_triangle};
mod pumpkin;
use pumpkin::{Pumpkin,Puddle};
mod game;
use game::Game;
mod scenes;
use scenes::{draw_mountains,rock_array, planting_view};

use crate::scenes::draw_mountains_intro;

#[allow(dead_code, unused)]
fn main() {
  
    Pumpkin::new(0, 1);
    let mut game = Game::new(); 
    let mut engine = console_engine::ConsoleEngine::init_fill(10).unwrap(); 
    let screen_width = engine.get_width() as i32;
    let puddle_a = Puddle::new(screen_width, 5);
    let puddle_b = Puddle::new(screen_width - 2, 0);
    let puddle_c = Puddle::new(screen_width - 2, 1);
    let mut puddle_d = Puddle::new(screen_width - 6, 4); 
    puddle_d.radius = 4;

    let mut puddles = vec![puddle_a, puddle_b, puddle_c, puddle_d];
    game.puddles = puddles;
    let mut left_intro = false;
    let mut frame = 0;
    loop{
        engine.wait_frame();
        engine.clear_screen();
        frame += 1;
        if engine.is_key_pressed(KeyCode::Esc)  {

            break;
        }
        //draw_mountains(&mut engine, frame, true, &rocks);
        //planting_view(&mut engine);
        if left_intro{

            game.run(&mut engine);
            game.add_time(1);
        }
        else{
            draw_mountains_intro(&mut engine, 1, false);
            title(&mut engine, frame);
            left_intro = engine.is_key_pressed(KeyCode::Char(' '));
            
        }
        engine.draw();
    }


}

fn title(engine: &mut ConsoleEngine, frame:i32){
    let orig_message = "The Life and Times";
    let sec_ms ="of Michael K.";
    let hold_message: String = orig_message.chars().take(frame as usize).collect();
    let my_message = &hold_message;
    let  width:i32 = (engine.get_width()).try_into().unwrap();
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert(my_message).unwrap();
    //assert!(figure.is_some());
    let print_str = &format!("{}",figure);
    //engine.print((width/2) - (print_str.len().try_into().unwrap_or(0)),0,&print_str );
    let text_width: i32  = my_message.len().try_into().unwrap();
    let start_x = (width/2) - text_width *3;
    let start_y = 3;
    let padding = 0;
    engine.print_fbg(start_x,start_y,&print_str , Color::Grey, Color::AnsiValue(236));
    let hold_message: String = sec_ms.chars().take((frame as usize - orig_message.chars().count()).max(1 )).collect();
    let my_message = &hold_message;
    let figure = standard_font.convert(my_message).unwrap();
    //assert!(figure.is_some());
    let print_str = &format!("{}",figure);
    //engine.fill_rect( start_x - padding, start_y - padding , start_x + text_width  + padding, start_y + figure.height as i32 + padding, pixel::pxl_bg(' ', Color::Black));
    if frame > orig_message.len() as i32 {
            engine.print_fbg(start_x + 16,start_y + 8,&print_str , Color::Grey, Color::AnsiValue(236));
            engine.print_fbg(width/2 -10,start_y + 16,"A game by Mackenzie Norman\nInspired by J. M. Coetzee" , Color::Grey, Color::AnsiValue(236));
        if  frame %4 != 0{
            engine.print_fbg(width/2 - 12, start_y + 24 + figure.height as i32, "Press Space to start", Color::Black, Color::AnsiValue(94));
        }
    }
}
