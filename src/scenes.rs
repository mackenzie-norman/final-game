use std::collections::btree_map::Range;

use console_engine::pixel::{self, Pixel};
use console_engine::rect_style::BorderStyle;
use console_engine::{screen, Color, MouseButton};
use console_engine::ConsoleEngine;
use console_engine::KeyCode;
use figlet_rs::FIGfont;
use rand::Rng;

pub fn draw_mountains(engine: &mut ConsoleEngine, frame:i32, day:bool, rocks: &Vec<(i32,i32,i32)> ){
    let screen_height = engine.get_height() as i32;
    let screen_width = engine.get_width() as i32; 
    let skyline = screen_height/4 + screen_height/6;
    let soil_line = screen_height / 2 + screen_height/4;
    let mut  sky = pixel::pxl_bg(' ', Color::AnsiValue(236));
    if day{

        sky = pixel::pxl_bg(' ', Color::AnsiValue(81));
    }
        

    let hill_color_a = Color::AnsiValue(94);
    let hill_color_b = Color::AnsiValue(100);
    let hill_color_c = Color::AnsiValue(143);
    let hill_color_d = Color::AnsiValue(144);
    let hill_color_e = Color::AnsiValue(151);
    
    let hill_px_1 = pixel::pxl_fg('#', hill_color_a);
    let hill_px_2 = pixel::pxl_fg('#', hill_color_b);
    let hill_px_3 = pixel::pxl_fg('#', hill_color_c);
    let hill_px_4 = pixel::pxl_fg('#', hill_color_d);
    let hill_px_5 = pixel::pxl_fg('#', hill_color_e);
    engine.fill_rect(0, screen_height, screen_width, skyline, hill_px_1);
    engine.fill_rect(0, 0, screen_width, skyline, sky);
    //boob 1
    engine.fill_circle(screen_width, screen_height + screen_width/4 + 2, (screen_width/2).try_into().unwrap(), hill_px_2);
    engine.fill_circle(screen_width +2, screen_height + screen_width/4 + 2, (screen_width/2).try_into().unwrap(), hill_px_4);
    engine.fill_circle(screen_width +4, screen_height + screen_width/4 + 2, (screen_width/2).try_into().unwrap(), hill_px_2);
    engine.fill_circle(screen_width +2, screen_height + screen_width/4 + 4, (screen_width/2).try_into().unwrap(), hill_px_3);
    
    engine.fill_circle(screen_width/4, screen_height + screen_width/4 + 2, (screen_width/2).try_into().unwrap(), hill_px_2);
    engine.fill_rect(0, screen_height, screen_width, soil_line  , hill_px_1);
    damn(engine,  screen_width/2 + screen_width/8  , soil_line , screen_width - screen_width/4, soil_line  + 3);
    //draw sky last
    for rck in rocks{
        let mut rng = rand::rng();
        let height = rck.2;
        let y = rck.1;
        rock(engine, frame, rck.0 ,  y, 1, y + height);
    }
    
}
fn damn(engine: &mut ConsoleEngine, x1: i32,y1:i32, x2: i32,y2:i32){
    engine.fill_rect(x1  , y1 -1, x2 , y2 -1 , pixel::pxl_fg('@', Color::DarkBlue));
    engine.fill_rect(x1  , y1, x2 , y2 , pixel::pxl_fg('#', Color::Grey));
}
fn rock(engine: &mut ConsoleEngine, frame:i32 , x1: i32,y1:i32,scale: i32, ground: i32){
    let height = 4 * scale;
    let width = 2* scale;
    engine.fill_rect(x1 + (width/2) , y1, x1 +width + (width/2) , ground , pixel::pxl_fg('#', Color::Grey));
}
pub fn rock_array(engine: &mut ConsoleEngine, count: i32) -> Vec<(i32,i32,i32)>{
    let screen_height = engine.get_height() as i32;
    let screen_width = engine.get_width() as i32; 
    
    let soil_line = screen_height / 2 + screen_height/4;
    let mut rng = rand::rng();
    let mut  v: Vec<(i32,i32,i32)> = Vec::new();
    for _ in 0..count{
        let height = rng.random_range(1..4);
        let y = rng.random_range(soil_line .. screen_height);
        let x = rng.random_range(0..screen_width);
        v.push((x,y,height));
    }
    return v;


}

pub fn planting_view(engine: &mut ConsoleEngine){
    let screen_height = engine.get_height() as i32;
    let screen_width = engine.get_width() as i32;
    
    let hill_color_a = Color::AnsiValue(94);
    let hill_color_b = Color::AnsiValue(100);
    let hill_color_c = Color::AnsiValue(143);
    let hill_color_d = Color::AnsiValue(144);
    let hill_color_e = Color::AnsiValue(151);
    
    let hill_px_1 = pixel::pxl_fg('#', hill_color_a);
    let hill_px_2 = pixel::pxl_fg('#', hill_color_b);
    let hill_px_3 = pixel::pxl_fg('#', hill_color_c);
    let hill_px_4 = pixel::pxl_fg('#', hill_color_d);
    let hill_px_5 = pixel::pxl_fg('#', hill_color_e);
    engine.fill_rect(0, screen_height, screen_width, 0  , hill_px_1);
}