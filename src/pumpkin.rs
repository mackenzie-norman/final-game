use rand::{random, Rng, rngs::StdRng, SeedableRng};
use console_engine::pixel::{self, Pixel};
use console_engine::rect_style::BorderStyle;
use console_engine::{screen, Color, MouseButton};
use console_engine::ConsoleEngine;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pumpkin{
    pub growth_stage: u8,
    pub water_lvl: i32,
    x: i32,
    y:i32,  
}
impl Pumpkin{
    pub fn new(x:i32, y:i32) -> Self{
        return Pumpkin{growth_stage : 0, water_lvl:0, x:x, y:y };
    }
    pub fn grow(&mut self){
        let mut rng = rand::rng();
        if rng.random_bool((self.water_lvl) as f64/100.0){
            self.growth_stage += 1;
        }
        if self.water_lvl > 1 && rng.random_bool(0.1){
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
            let water_color = pixel::pxl_bg(' ', Color::AnsiValue(94)); // Dark brown
            engine.fill_circle(self.x, self.y, water_radius, water_color);
        }
        let color = match self.growth_stage {
            0..=29 => pixel::pxl_fg('.', Color::AnsiValue(28)), // small green dot
            30..=59 => pixel::pxl_fg('o', Color::AnsiValue(166)), // orange circle
            _ => pixel::pxl_fg('O', Color::AnsiValue(166)),       // large orange
        };

        let radius = match self.growth_stage {
            0..=29 => 0,
            30..=59 => 1,
            _ => 2,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Melon{
    pub growth_stage: u8,
    pub water_lvl: i32,
    x: i32,
    y:i32,  
}
impl Melon{
    pub fn new(x:i32, y:i32) -> Self{
        return Melon{growth_stage : 0, water_lvl:0, x:x, y:y };
    }
    pub fn grow(&mut self){
        let mut rng = rand::rng();
        if rng.random_bool((self.water_lvl) as f64/100.0){
            self.growth_stage += 1;
        }
        if self.water_lvl > 1 && rng.random_bool(0.01){
            self.water_lvl -= 1;
        }
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
            let water_color = pixel::pxl_bg(' ', Color::AnsiValue(94)); // Dark brown
            engine.fill_circle(self.x, self.y, water_radius, water_color);
        }
        let color = match self.growth_stage {
            0..=29 => pixel::pxl_fg('*', Color::AnsiValue(28)), // small green dot
            30..=59 => pixel::pxl_fg('o', Color::AnsiValue(118)), // lighter green
            _ => pixel::pxl_fg('O', Color::AnsiValue(118)),       // large light green
        };

        let radius = match self.growth_stage {
            0..=29 => 0,
            30..=59 => 1,
            _ => 2,
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
}