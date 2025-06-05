use rand::rngs::ThreadRng;
use rand::{random, Rng, rngs::StdRng, SeedableRng};
use console_engine::pixel::{self, Pixel};
use console_engine::rect_style::BorderStyle;
use console_engine::{screen, Color, MouseButton};
use console_engine::ConsoleEngine;

use crate::scenes::planting_view;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pumpkin{
    pub growth_stage: u8,
    pub water_lvl: i32,
    pub x: i32,
    pub y:i32,  
}
impl Pumpkin{
    pub fn new(x:i32, y:i32) -> Self{
        return Pumpkin{growth_stage : 0, water_lvl:0, x:x, y:y };
    }
    pub fn grow(&mut self, rng: &mut ThreadRng){
        
        if rng.random_bool((self.water_lvl) as f64/100.0){
            self.growth_stage += 1;
        }
        if self.water_lvl > 1 && rng.random_bool(0.09){
            self.water_lvl -= 1;
        }

    }
    pub fn contains_coords(&self, x: i32, y: i32) -> bool {
        let dx = x - self.x;
        let dy = y - self.y;
        dx * dx + dy * dy <= 4  // 2^2 = 4
    }
    pub fn draw(self, engine: &mut ConsoleEngine) {
        let water_radius = match self.water_lvl {
            0 => 0,
            1..=10 => 1,
            11..=30 => 2,
            _ => 3,
        };
        // Dark brown water base (draw first, behind the plant)
        if water_radius > 0 {
            let water_color = pixel::pxl_bg(' ', Color::AnsiValue(235)); // Dark brown
            engine.fill_circle(self.x, self.y, water_radius, water_color);
        }
        let color = match self.growth_stage {
            0..=29 => pixel::pxl_fbg('o', Color::AnsiValue(28), Color::AnsiValue(94)), // small green dot
            30..=95 => pixel::pxl_fg('0', Color::AnsiValue(166)), // orange circle
            _ => pixel::pxl_fg('@', Color::AnsiValue(166)),       // large orange
        };

        let radius = match self.growth_stage {
            0..=29 => 0,
            30..=59 => 1,
            60..=75 => 2,
            _ => 3,
        };

        if radius == 0 {
            engine.set_pxl(self.x, self.y, color);
        } else {
            engine.fill_circle(self.x, self.y, radius, color);
        }
    }
    pub fn water(&mut self, amount: i32) {
        self.water_lvl = ((self.water_lvl + amount) % 51) + 1;
    }
    pub fn draw_at(&self, engine: &mut ConsoleEngine, x: i32, y: i32) {
    let color = match self.growth_stage {
        0..=29 => pixel::pxl_fbg('o', Color::AnsiValue(28), Color::AnsiValue(94)), // small green dot
        30..=59 => pixel::pxl_fg('O', Color::AnsiValue(166)), // orange circle
        _ => pixel::pxl_fg('@', Color::AnsiValue(166)),       // large orange
    };

    let radius = match self.growth_stage {
        0..=29 => 0,
        30..=59 => 1,
        60..=75 => 2,
        _ => 3,
    };

    if radius == 0 {
        engine.set_pxl(x, y, color);
    } else {
        engine.fill_circle(x, y, radius, color);
    }
    let vine = pixel::pxl_fbg('*', Color::AnsiValue(28), Color::AnsiValue(94)); // green vine with dark ground
    // Progressive vine drawing
    match self.growth_stage {
        0..=9 => {
            engine.set_pxl(x, y + 1, vine);
        }
        10..=29 => {
            engine.line(x, y, x + 1, y + 2, vine);
        }
        30..=59 => {
            engine.line(x, y, x + 1, y + 2, vine);
            engine.line(x + 1, y + 2, x + 3, y + 3, vine);
        }
        60..=79 => {
            engine.line(x, y, x + 1, y + 2, vine);
            engine.line(x + 1, y + 2, x + 3, y + 3, vine);
            engine.line(x - 1, y + 2, x - 3, y + 4, vine);
        }
        _ => {
            engine.line(x, y, x + 1, y + 2, vine);
            engine.line(x + 1, y + 2, x + 3, y + 3, vine);
            engine.line(x - 1, y + 2, x - 3, y + 4, vine);
            engine.line(x, y + 1, x, y + 5, vine);
        }
    }

    
    }
    pub fn is_ready(&self) -> bool{
        return self.growth_stage > 95;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Melon{
    pub growth_stage: u32,
    pub water_lvl: i32,
    pub x: i32,
    pub y:i32,  
}
impl Melon{
    pub fn new(x:i32, y:i32) -> Self{
        return Melon{growth_stage : 0, water_lvl:0, x:x, y:y };
    }
    pub fn grow(&mut self, rng: &mut ThreadRng){
        
        if rng.random_bool((self.water_lvl.min(1)) as f64/100.0){
            self.growth_stage += 1;
        }
        if self.water_lvl > 1 && rng.random_bool(0.09){
            self.water_lvl -= 1;
        }
    }
    pub fn is_ready(&self) -> bool{
        return self.growth_stage > 95;
    }
    pub fn contains_coords(&self, x: i32, y: i32) -> bool {
        let dx = x - self.x;
        let dy = y - self.y;
        dx * dx + dy * dy <= 4  // 2^2 = 4
    }
    pub fn draw(&self, engine: &mut ConsoleEngine) {
        let water_radius = match self.water_lvl {
            0 => 0,
            1..=10 => 1,
            11..=30 => 2,
            _ => 3,
        };

        // Dark brown water base (draw first, behind the plant)
        if water_radius > 0 {
            let water_color = pixel::pxl_bg(' ', Color::AnsiValue(235)); // Dark brown
            engine.fill_circle(self.x, self.y, water_radius, water_color);
        }
        let color = match self.growth_stage {
            0..=29 => pixel::pxl_fg('*', Color::AnsiValue(28)), // small green dot
            30..=59 => pixel::pxl_fg('o', Color::AnsiValue(118)), // lighter green
            _ => pixel::pxl_fg('0', Color::AnsiValue(118)),       // large light green
        };

        let radius = match self.growth_stage {
            0..=29 => 0,
            30..=59 => 1,
            60..=75 => 2,
            _ => 3,
        };

        if radius == 0 {
            engine.set_pxl(self.x, self.y, color);
        } else {
            engine.fill_circle(self.x, self.y, radius, color);
        }
    }
    pub fn water(&mut self, amount: i32) {
        self.water_lvl = ((self.water_lvl + amount) % 51) + 1;
    }

    pub fn draw_at(&self, engine: &mut ConsoleEngine, x: i32, y: i32) {
    let color = match self.growth_stage {
        0..=29 => pixel::pxl_fg('*', Color::AnsiValue(28)),      // early: green sprout
        30..=59 => pixel::pxl_fg('o', Color::AnsiValue(118)),    // mid: small fruit
        _ => pixel::pxl_fg('O', Color::AnsiValue(118)),          // full: large fruit
    };

    let radius = match self.growth_stage {
        0..=29 => 0,
        30..=59 => 1,
        60..=75 => 2,
        _ => 3,
    };

    let vine = pixel::pxl_fbg('*', Color::AnsiValue(28), Color::AnsiValue(94)); // green vine with dark ground
    // Progressive vine drawing
    match self.growth_stage {
        0..=9 => {
            engine.set_pxl(x, y + 1, vine);
        }
        10..=29 => {
            engine.line(x, y, x + 1, y + 2, vine);
        }
        30..=59 => {
            engine.line(x, y, x + 1, y + 2, vine);
            engine.line(x + 1, y + 2, x + 3, y + 3, vine);
        }
        60..=79 => {
            engine.line(x, y, x + 1, y + 2, vine);
            engine.line(x + 1, y + 2, x + 3, y + 3, vine);
            engine.line(x - 1, y + 2, x - 3, y + 4, vine);
        }
        _ => {
            engine.line(x, y, x + 1, y + 2, vine);
            engine.line(x + 1, y + 2, x + 3, y + 3, vine);
            engine.line(x - 1, y + 2, x - 3, y + 4, vine);
            engine.line(x, y + 1, x, y + 5, vine);
        }
    }
    if radius == 0 {
        engine.set_pxl(x, y, color);
    } else {
        engine.fill_circle(x, y, radius, color);
    }

    
}
}

pub struct Puddle {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
}

impl Puddle {
    pub fn new(x: i32, y: i32) -> Self {
        Puddle { x, y, radius: 2 }
    }

    pub fn draw(&self, engine: &mut ConsoleEngine) {
        let water_color = pixel::pxl_fg('~', Color::AnsiValue(33)); // Light blue
        engine.fill_circle(self.x, self.y, self.radius.try_into().unwrap(), water_color);
    }

    pub fn contains_coords(&self, x: i32, y: i32) -> bool {
        let dx = x - self.x;
        let dy = y - self.y;
        dx * dx + dy * dy <= self.radius * self.radius
    }
}

pub struct Weed {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
}

impl Weed {
    pub fn new(x: i32, y: i32) -> Self {
        Weed { x, y, radius: 1 }
    }
    pub fn grow(&mut self, rng: &mut ThreadRng){
        
        if rng.random_bool(0.1){
            self.radius = (self.radius + 1) % 3;
        }
        
    }
    pub fn draw(&self, engine: &mut ConsoleEngine) {
        let vine = pixel::pxl_fbg('X', Color::AnsiValue(28), Color::AnsiValue(94));
        if self.radius != 0{

            engine.fill_circle(self.x, self.y, self.radius.try_into().unwrap(), vine);
        }
        else{
            engine.set_pxl(self.x, self.y,  vine); 
        }
    }
    pub fn draw_at(&self, engine: &mut ConsoleEngine, x: i32, y: i32) {
    

    let vine = pixel::pxl_fbg('*', Color::AnsiValue(28), Color::AnsiValue(94)); // green vine with dark ground
    // Progressive vine drawing
    match self.radius {
        0=> {
            engine.line(x, y, x + 1, y + 2, vine);
        }
        1 => {
            engine.line(x, y, x + 1, y + 2, vine);
            engine.line(x + 1, y + 2, x + 3, y + 3, vine);
        }
        2 => {
            engine.line(x, y, x + 1, y + 2, vine);
            engine.line(x + 1, y + 2, x + 3, y + 3, vine);
            engine.line(x - 1, y + 2, x - 3, y + 4, vine);
        }
        _ => {
            engine.line(x, y, x + 1, y + 2, vine);
            engine.line(x + 1, y + 2, x + 3, y + 3, vine);
            engine.line(x - 1, y + 2, x - 3, y + 4, vine);
            engine.line(x, y + 1, x, y + 5, vine);
        }
    }
    }
    pub fn contains_coords(&self, x: i32, y: i32) -> bool {
        let dx = x - self.x;
        let dy = y - self.y;
        dx * dx + dy * dy <= self.radius * self.radius
    }
}