use std::fmt::format;
use std::io;
use std::sync::mpsc::RecvTimeoutError;
use rand::{random, Rng, rngs::StdRng, SeedableRng};
use console_engine::pixel::{self, Pixel};
use console_engine::rect_style::BorderStyle;
use console_engine::{screen, Color, MouseButton};
use console_engine::ConsoleEngine;
use console_engine::KeyCode;
use figlet_rs::FIGfont;
use crate::dialouge::{Dialouge, pt_in_box};
use crate::{character, pumpkin};
use character::Character;
use crate::debug_engine;
use crate::pumpkin::{Pumpkin, Melon};
use crate::game::HandOptions::{Clicker,Water, PumpkinSeeds, MelonSeeds};
use crate::scenes::{draw_mountains};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum HandOptions {
    Clicker,
    Water,
    PumpkinSeeds,
    MelonSeeds,
}
pub struct Game{
    hunger  : u32,
    pumpkins : Vec<Pumpkin>,
    melons : Vec<Melon>,
    p_seeds : u32,
    m_seeds : u32,
    in_hand: HandOptions,
    topview : bool
}
impl Game{
    pub fn new() -> Self{
        return  Game{hunger :100, pumpkins: vec![], melons : vec![], in_hand: Clicker, p_seeds:10, m_seeds:2, topview:true};
    }
    pub fn menu(&mut self, engine: &mut ConsoleEngine) {
    let screen_height = engine.get_height() as i32;
    let box_size = screen_height / 8;

    let mut menu_bounds = Vec::new();

    for i in 0..5 {
        let y1 = 1 + (i * box_size);
        let y2 = y1 + box_size;
        let x1 = 1;
        let x2 = x1 + box_size;

        engine.rect_border(x1, y1, x2, y2, BorderStyle::new_simple());

        // Draw seed counts for PumpkinSeeds (2) and MelonSeeds (3)
        match i {
            2 => {
                engine.print(x1 +1 , y2 - 1, &format!("Seeds: {}", self.p_seeds));
            }
            3 => {
                engine.print(x1+1, y2 - 1, &format!("Seeds: {}", self.m_seeds));
            }
            _ => {}
        }

        menu_bounds.push((x1, x2, y1, y2));
    }

    if let Some((mx, my)) = engine.get_mouse_press(MouseButton::Left) {
        let mx = mx as i32;
        let my = my as i32;

        let mut clicked_menu = false;

        for (i, (x1, x2, y1, y2)) in menu_bounds.iter().enumerate() {
            if mx >= *x1 && mx <= *x2 && my >= *y1 && my <= *y2 {
                if i  == 0 {self.topview = !self.topview};
                self.in_hand = match i {
                    0 => HandOptions::Clicker,
                    1 => HandOptions::Water,
                    2 => HandOptions::PumpkinSeeds,
                    3 => HandOptions::MelonSeeds,
                    _ => HandOptions::Clicker,
                };
                clicked_menu = true;
                break;
            }
        }

        if !clicked_menu {
            self.try_plant(mx, my);
            self.try_water(mx, my);
            self.try_harvest(mx, my);
        }
    }
    for pumpkin in &mut self.pumpkins {
        pumpkin.draw(engine);
        pumpkin.grow();
    }

    for melon in &mut self.melons {
        melon.draw(engine);
        melon.grow();

    }
    }
    pub fn is_tile_occupied(&self, x: i32, y: i32) -> bool {
        self.pumpkins.iter().any(|p| p.contains_coords(x, y)) ||
        self.melons.iter().any(|m| m.contains_coords(x, y))
    }   
    pub fn try_plant(&mut self, x: i32, y: i32) {
        if self.is_tile_occupied(x, y) {
            return;
        }
        match self.in_hand {
            HandOptions::PumpkinSeeds => {
                if self.p_seeds > 0 {
                    self.pumpkins.push(Pumpkin::new(x, y));
                    self.p_seeds -= 1;
                }
            }
            HandOptions::MelonSeeds => {
                if self.m_seeds > 0 {
                    self.melons.push(Melon::new(x, y));
                    self.m_seeds -= 1;
                }
            }
            _ => {} // Clicker and Water do nothing
        }
    }
    pub fn try_water(&mut self, x: i32, y: i32) {
    if self.in_hand != HandOptions::Water {
        return;
    }

    for pumpkin in &mut self.pumpkins {
        if pumpkin.contains_coords(x, y) {
            pumpkin.water(20);
            return;
        }
    }

    for melon in &mut self.melons {
        if melon.contains_coords(x, y) {
            melon.water(20);
            return;
        }
    }
}

pub fn try_harvest(&mut self, x: i32, y: i32) {
    if self.in_hand != HandOptions::Clicker {
        return;
    }
    
    if let Some(index) = self.pumpkins.iter().position(|p| p.contains_coords(x, y) && p.growth_stage >= 60) {
        let mut rng = rand::rng();
        let seeds_gained = rng.random_range(1..=3);
        self.p_seeds += seeds_gained;
        self.pumpkins.remove(index);
        
        self.hunger = self.hunger  + 20;
        return;
    }

    if let Some(index) = self.melons.iter().position(|m| m.contains_coords(x, y) && m.growth_stage >= 60) { 
        let mut rng = rand::rng();
        let seeds_gained = rng.random_range(1..=3);
        self.m_seeds += seeds_gained;
        self.melons.remove(index);
        self.hunger = self.hunger  + 20;
        return;
    }
}

pub fn draw_landscape(engine: &mut ConsoleEngine, land_y1:i32){


}


    
}
fn top_down_tracks(engine: &mut ConsoleEngine, frame:i32 , x1:i32, y1: i32, x2: i32, y2:i32){
    let rail_char = pixel::pxl_fg('#', Color::AnsiValue(242));
    let rail_char = pixel::pxl_fg('#', Color::AnsiValue(242));
    let dirt_char = pixel::pxl_fg('@', Color::AnsiValue(58));
    let track_width = 2;
    let tie_width = 2;
    //engine.fill_rect(x1 + frame, y1-4, x2 + frame, y2 + 4, dirt_char);
    engine.fill_rect(x1 + frame, y1 +1 , x2 + frame, y1 + track_width, rail_char);
    engine.fill_rect(x1 + frame, y2 -1 , x2 + frame, y2 - track_width, rail_char);
    for i in x1..=x2{
        if i % (tie_width * 4) == 0{
            engine.fill_rect(x1 + i + frame, y1 - track_width , x1 + i + frame + tie_width , y2 + track_width, rail_char);
        }

    }
}
fn draw_platform(engine: &mut ConsoleEngine, frame:i32, height:i32){
    let platform_color = Color::AnsiValue(236);
    let platform_char = pixel::pxl_fbg(' ', platform_color, platform_color);
    let split_char = pixel::pxl_fbg('|', Color::Black, platform_color);
    let screen_height = engine.get_height() as i32;
    let screen_width = engine.get_width() as i32;
    engine.fill_rect(0, height, screen_width,screen_height, platform_char);
    let spacing = screen_width/12;
    for i in (0..screen_width).into_iter().step_by(spacing as usize){

        engine.line(i, height, i, screen_height, split_char);
    }
}
fn barcode(engine: &mut ConsoleEngine, boxx: (i32,i32,i32,i32)){
    let array = [true, false,true,false,false,true];
    for i in boxx.1..= boxx.3{
        if array[(i% array.len() as i32) as usize ]{
            if i % 4 == 0 {
                engine.line(boxx.0, i, boxx.2, i, pixel::pxl('-'));
            }else{

                engine.line(boxx.0, i, boxx.2, i, pixel::pxl('='));
            }
        }
    }
}
fn confirm(engine: &mut ConsoleEngine, name:String){
    loop{
    engine.wait_frame();
    let screen_width: i32 =(engine.get_width()) as i32;
    let screen_height: i32 =(engine.get_height()) as i32;

    let bg_char = pixel::pxl_bg(' ', Color::Black);
    let box_x1: i32 = screen_width/6;
    let box_x2: i32 = screen_width - box_x1;
     
    let box_y1 = screen_height/3 + screen_height/3 + screen_height/24;// + screen_height/36;
    let box_y2 = screen_height - screen_height/6 + screen_height/24;
        //TODO chunk chars to pages 
    engine.fill_rect(box_x1, box_y1, box_x2, box_y2, bg_char);
    engine.rect_border(box_x1, box_y1, box_x2, box_y2, BorderStyle::new_heavy());
    let print_str: String = format!("Are you sure {} is your name? (press enter to confirm)", name);
    engine.print(box_x1 + 1, box_y1 + 1,&print_str );
    if engine.is_key_pressed(KeyCode::Enter ){
        break;
    }
    engine.draw();

    }
}
fn get_text(engine: &mut ConsoleEngine, ) -> Option<char>{
    let lower_case = ('a'..='z').into_iter().collect::<Vec<char>>();
    let upper_case = ('A'..='Z').into_iter().collect::<Vec<char>>();
    for ch in lower_case{
        if engine.is_key_pressed(KeyCode::Char(ch)){
            return Some(ch);
        }
    }
    for ch in upper_case{
        if engine.is_key_pressed(KeyCode::Char(ch)){
            return Some(ch);
        }
    }
    if engine.is_key_pressed(KeyCode::Char(' ')){
        return Some(' ');
    }
    return None; 
}
