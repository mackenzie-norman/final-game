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
use pumpkin::Pumpkin;
mod game;
use game::Game;
mod scenes;
use scenes::{draw_mountains,rock_array, planting_view};

#[allow(dead_code, unused)]
fn main() {
    
    Pumpkin::new(0, 1);
    let mut game = Game::new(); 
    let mut engine = console_engine::ConsoleEngine::init_fill(20).unwrap();
    let rocks = rock_array(&mut engine, 20);
    let mut frame = 0;
    loop{
        engine.wait_frame();
        engine.clear_screen();
        if engine.is_key_pressed(KeyCode::Esc)  {

            break;
        }
        //draw_mountains(&mut engine, frame, true, &rocks);
        planting_view(&mut engine);
        game.menu(&mut engine);
        engine.draw();
        frame += 1;
    }

}
