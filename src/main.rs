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
#[allow(dead_code, unused)]
fn main() {
    println!("Hello, world!");
}
