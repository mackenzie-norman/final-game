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
use crate::pumpkin::{Melon, Puddle, Pumpkin};
use crate::game::HandOptions::{Clicker,Water, PumpkinSeeds, MelonSeeds};
use crate::scenes::{draw_mountains, planting_view, rain};
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
    topview : bool,
    water : i32,
    pub puddles : Vec<Puddle>,
    time:i32,
}
impl Game{
    pub fn new() -> Self{
        
        return  Game{hunger :100, pumpkins: vec![], melons : vec![], in_hand: Clicker, p_seeds:10, m_seeds:2, topview:true, water : 4, puddles: vec![], time:0};
    }
    pub fn run(&mut self, engine: &mut ConsoleEngine ,){
        if self.topview{
            self.menu(engine);
        }else{
            self.draw_landscape(engine);
        }
        if self.time > 400 && self.time < 800{
            rain(engine, self.time );
            for pumpkin in &mut self.pumpkins {
                pumpkin.water(1);
            }

            for melon in &mut self.melons {
                melon.water(1);
            }
        }
    }
    pub fn menu(&mut self, engine: &mut ConsoleEngine) {
    let screen_height = engine.get_height() as i32;
    
    planting_view(engine);
    let box_size = screen_height / 8;

    let mut menu_bounds = Vec::new();

    for i in 0..5 {
    let y1 = 1 + (i * box_size);
    let y2 = y1 + box_size -1;
    let x1 = 1;
    let x2 = x1 + box_size;

    // Highlight active box
    let border_style = if match i {
        0 => self.in_hand == HandOptions::Clicker,
        1 => self.in_hand == HandOptions::Water,
        2 => self.in_hand == HandOptions::PumpkinSeeds,
        3 => self.in_hand == HandOptions::MelonSeeds,
        _ => false,
    } {
        BorderStyle::new_simple().with_colors(Color::Green, Color::Black)
    } else {
        BorderStyle::new_simple()
    };

    engine.rect_border(x1, y1, x2, y2, border_style);

    // Content based on box index
    match i {
        0 => {
            // Clicker icon
            let handle = pixel::pxl_fg('+', Color::AnsiValue(130));
            let blade = pixel::pxl_fg('#', Color::AnsiValue(240));
            engine.line(x1 + 1, y2 - 1, x2 - 1, box_size / 2, handle);
            engine.line(x2 - 1, box_size / 2, x1 + 2, y1 + 2, blade);
        }
        1 => {
            // Water icon
            let water_color = pixel::pxl_fg('~', Color::AnsiValue(33));
            for j in 0..self.water.min(6) {
                engine.line(x1 + 1, y2 - 1 - j, x2 - 1, y2 - 1 - j, water_color);
            }
            engine.print_fbg(x1 + 1, y2 - 1, "Water", Color::AnsiValue(33), Color::Black);
        }
        2 => {
            // Pumpkin seeds + icon
            let pumpkin_color = pixel::pxl_bg(' ', Color::AnsiValue(166));  // orange
            let stem_color = pixel::pxl_fbg('*', Color::AnsiValue(22), Color::AnsiValue(166));      // dark green
            let cx = (x1 + x2) / 2;
            let cy = (y1 + y2) / 2;

            engine.fill_circle(cx, cy, 2, pumpkin_color); // pumpkin body
            engine.set_pxl(cx, cy , stem_color); // stem
            
            engine.print(x1 + 1, y2 - 1, &format!("Seeds: {}", self.p_seeds));
        }
        3 => {
            // Melon seeds + icon
            let pumpkin_color = pixel::pxl_bg(' ', Color::AnsiValue(34));  // orange
            let stem_color = pixel::pxl_fbg('*', Color::AnsiValue(22), Color::AnsiValue(34));      // dark green
            let cx = (x1 + x2) / 2;
            let cy = (y1 + y2) / 2;

            engine.fill_circle(cx, cy, 2, pumpkin_color); // pumpkin body
            engine.set_pxl(cx, cy , stem_color); // stem
            
            engine.print(x1 + 1, y2 - 1, &format!("Seeds: {}", self.m_seeds));
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
                if i  == 4 {self.topview = !self.topview};
                self.in_hand = match i {
                    0 => HandOptions::Clicker,
                    1 => HandOptions::Water,
                    2 => HandOptions::PumpkinSeeds,
                    3 => HandOptions::MelonSeeds,
                    _ => {self.topview = false;  HandOptions::Clicker},
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
    for puddle in &mut self.puddles {
        puddle.draw(engine);
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
        if self.water <= 0{break;}
        if pumpkin.contains_coords(x, y) {
            pumpkin.water(20);
            self.water -= 1;
            return;
        }
    }

    for melon in &mut self.melons {
        if self.water <= 0{break;}
        if melon.contains_coords(x, y) {
            melon.water(20);
            self.water -= 1;
            return;
        }
    }
    for puddle in &mut self.puddles {
        if puddle.contains_coords(x, y){
            self.water = 6;
            return;
        }
    }
}

pub fn try_harvest(&mut self, x: i32, y: i32) {
    if self.in_hand != HandOptions::Clicker {
        return;
    }
    
    if let Some(index) = self.pumpkins.iter().position(|p| p.contains_coords(x, y) && p.is_ready()) {
        let mut rng = rand::rng();
        let seeds_gained = rng.random_range(1..=3);
        self.p_seeds += seeds_gained;
        self.pumpkins.remove(index);
        
        self.hunger = self.hunger  + 20;
        return;
    }

    if let Some(index) = self.melons.iter().position(|m| m.contains_coords(x, y) && m.is_ready()) { 
        let mut rng = rand::rng();
        let seeds_gained = rng.random_range(1..=3);
        self.m_seeds += seeds_gained;
        self.melons.remove(index);
        self.hunger = self.hunger  + 20;
        return;
    }
}

pub fn draw_landscape(&mut self, engine: &mut ConsoleEngine){

    draw_mountains(engine, 0, self.time < 600,);
    let screen_width = engine.get_width() as i32;
    let screen_height = engine.get_height() as i32;

    // Arrow sizing relative to screen
    let arrow_width = screen_width / 20; // ~10% of screen width
    let arrow_height = screen_height / 10; // ~10% of screen height

    // Arrow position (top-left corner)
    let ax = 1;
    let ay = arrow_height/2 + 1 ;

    let bx = arrow_width;
    let by = 1;

    let cx = arrow_width;
    let cy = arrow_height;

    let arrow_color = pixel::pxl_bg(' ', Color::AnsiValue(82)); // greenish

    // Draw the triangle arrow
    engine.fill_triangle(ax, ay, bx, by, cx, cy, arrow_color);
    engine.fill_rect(arrow_width, arrow_height - 2, arrow_width * 2 , 3, arrow_color);

    // Bounding box for click detection
    let min_x = 1;
    let max_x = arrow_width * 2;
    let min_y = 1;
    let max_y = arrow_height;

    if let Some((mx, my)) = engine.get_mouse_press(MouseButton::Left) {
        let mx = mx as i32;
        let my = my as i32;

        if mx >= min_x && mx <= max_x && my >= min_y && my <= max_y {
            self.topview = true;
        }
    }
    //loop through and draw pumpkins and melons from screen height to soil line using draw_at function and converting their y value to within the range
    let soil_line = screen_height / 2 + screen_height/4;
    let ground_bottom = screen_height;
     // Convert world Y (0–100) to screen Y
    let map_y = |y: i32| -> i32 {
        let y_clamped = y.clamp(0, 100);
        ground_bottom - ((y_clamped * (ground_bottom - soil_line)) / 100)
    };

    for pumpkin in &mut self.pumpkins {
        pumpkin.draw_at(engine, pumpkin.x, map_y(pumpkin.y));
        pumpkin.grow();
    }

    for melon in &mut self.melons {
        melon.draw_at(engine, melon.x, map_y(melon.y));
        melon.grow();
    }
}
pub fn add_time(&mut self, toadd:i32){
    self.time = (self.time + toadd) % 1200
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
