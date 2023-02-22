use pikuseru_rs::api;

use crate::universe::Universe;
use crate::species::{Species, self};

/*
Wall :Indestructible.
Sand :Sinks in water.
Water :Puts out fire.
Stone :Forms arches, turns into sand under pressure.
Ice :Freezes Water, slippery!
Gas :Highly Flammable!
Cloner :Copies the first element it touches.
Mite :Eats wood and plant, but loves dust! Slides on ice..
Wood :Sturdy, but biodegradable.
Plant :Thrives in wet enviroments.
Fungus :Spreads over everything.
Seed :Grows on sand, plant, and fungus.
Fire :Hot!
Lava :Flammable and heavy.
Acid :Corrodes other elements.
Dust :Pretty, but dangerously explosive.
Oil :Produces smoke when set on fire.
Rocket :Explodes into copies of the first element it touches.
Empty :Erases.
*/

pub struct RGB {
    /// The red component (0..1)
    pub r: f32,
    /// The green components (0..1)
    pub g: f32,
    /// The blue component (0..1)
    pub b: f32,
}

impl RGB {
    pub fn new() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn from_f32(r: f32, g: f32, b: f32) -> Self {
        let r_clamped = f32::min(1.0, f32::max(0.0, r));
        let g_clamped = f32::min(1.0, f32::max(0.0, g));
        let b_clamped = f32::min(1.0, f32::max(0.0, b));
        Self {
            r: r_clamped,
            g: g_clamped,
            b: b_clamped,
        }
    }
}

pub struct HSV {
    /// Hue (range 0..1)
    pub h: f32,
    /// Saturation (range 0..1)
    pub s: f32,
    /// Value (range 0..1)
    pub v: f32,
}

impl HSV {
    pub fn new() -> Self {
        Self {
            h: 0.0,
            s: 0.0,
            v: 0.0,
        }
    }

    pub fn to_rgb(&self) -> RGB {
        let h = self.h;
        let s = self.s;
        let v = self.v;

        let mut r: f32 = 0.0;
        let mut g: f32 = 0.0;
        let mut b: f32 = 0.0;

        let i = f32::floor(h * 6.0) as i32;
        let f = h * 6.0 - i as f32;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);

        match i % 6 {
            0 => {
                r = v;
                g = t;
                b = p;
            }
            1 => {
                r = q;
                g = v;
                b = p;
            }
            2 => {
                r = p;
                g = v;
                b = t;
            }
            3 => {
                r = p;
                g = q;
                b = v;
            }
            4 => {
                r = t;
                g = p;
                b = v;
            }
            5 => {
                r = v;
                g = p;
                b = q;
            }
            // Catch-all; this shouldn't happen
            _ => {}
        }

        RGB::from_f32(r, g, b)
    }
}

pub fn species_to_rgb(species: Species, ra: u8, rb:u8) -> RGB {
    let mut hue: f32 = 0.0;
    let mut saturation: f32 = 0.6;
    let mut lightness: f32 = 0.3 + (ra as f32/255.0) * 0.5;

    match species {
        Species::Empty => {
            hue = 0.10;
            saturation = 0.0;
            lightness = 0.18;
        }
        Species::Acid => {
            hue = 0.18;
            saturation = 0.9;
            lightness = 0.8 + (ra as f32 / 255.0) * 0.2;// + noise * 0.05;
        }
        Species::Fire => {  
            hue = (ra as f32 / 255.0) * 0.1;
            saturation = 0.7;
            lightness = 0.7 + ((ra as f32 / 255.0) * 0.3); // + ((noise + 0.8) * 0.5);
        }
        Species::Gas => {
            hue = 0.0;
            lightness += 0.4;
            saturation = 0.2 + ((ra as f32 / 255.0) * 1.5);
        }
        Species::Ice => {
            hue = 0.6;
            saturation = 0.4;
            lightness = 0.7 + (ra as f32 / 255.0) * 0.5;
        }
        Species::Lava => {
            hue = (ra as f32 / 255.0) * 0.1;
            lightness = 0.7 + (ra as f32 / 255.0) * 0.25;// + noise * 0.1;
        }
        Species::Mite => {
            hue = 0.8;
            saturation = 0.9;
            lightness = 0.8;
        }
        Species::Oil => {
            hue = ((ra as f32 / 255.0) * 5.0);// + t * 0.008;
            saturation = 0.2;
            lightness = 0.3;
        }
        Species::Plant => {
            hue = 0.4;
            saturation = 0.4;
        }
        Species::Rocket => {
            hue = 0.0;
            saturation = 0.4 + (rb as f32 / 255.0);
            lightness = 0.9;
        }
        Species::Sand => {
            hue = 0.1;
            saturation = 0.24;
            lightness = 0.87;
        }
        Species::Stone => {
            hue = 0.58 + ((ra as f32/255.0) * 0.5);
            saturation = 0.1;
        }
        Species::Seed => {
            hue = ((rb as f32 / 255.0 * 2.) * 0.5);// - 0.3;
            saturation = 0.7 * (ra as f32 / 255.0 + 0.4) + rb as f32 / 255.0 * 0.2;
            lightness = 0.9 * (ra as f32 / 255.0 + 0.9);
        }
        Species::Water => {
            hue = 0.6;
            lightness = 0.7 + (ra as f32/255.0) * 0.25; // + noise * 0.1;
        }
        Species::Wood => {
            hue = (ra as f32/255.0) * 0.1;
            saturation = 0.3;
            lightness = 0.3 + (ra as f32 /255.0) * 0.3;
        }
        _ => {}
    }

    if species != Species::Empty {
        lightness *= 0.975 + api::frnd();
    }

    return HSV{h: hue, s: saturation, v: lightness}.to_rgb();
}

#[cfg(test)]
mod tests {
    use super::species_to_rgb;
    use super::HSV;
    use crate::species::Species;

    #[test]
    fn test_color() {
        let rgbcolor = species_to_rgb(Species::Empty, 0, 0);
        println!("RGB {:?} {:?} {:?}", rgbcolor.r, rgbcolor.g, rgbcolor.b);
    }
}

pub struct Widget {
    species: Species,
    name: String,
    idx: u32,
    x: u32,
    y: u32,
    col: u32,
    active: bool,
}

impl Widget {
    pub fn new(species: Species, name: String, idx: u32, x: u32, y: u32, col: u32, active: bool) -> Self {
        Self {
            species: species,
            name: name,
            idx: idx,
            x: x,
            y: y,
            col: col,
            active: active
        }
    }

    pub fn update(&mut self, left_click: bool, mouse_x: u32, mouse_y: u32) -> bool {
        if left_click {
            if mouse_x >= self.x && mouse_x <= self.x+8 {
                if mouse_y >= self.y && mouse_y <= self.y+8 {
                    self.active = true;
                    return true;
                }
            }
            self.active = false;
        }
        return false;
    }

    pub fn draw(&mut self) {

        if self.active {
            api::print(self.name.as_str(), 0, 260, 7);
            api::rectfill(self.x as i32, self.y as i32, self.x as i32 +8, self.y as i32 + 8, 7);
        }

        api::spr(self.idx, self.x as i32, self.y as i32, 1, 1, false, false, 0.0, 1.0, false);

        //
        //api::rectfill(self.x, self.y, self.x + 10, self.y+5, self.col);
    }
}
pub struct MyGame {
    universe: Universe,
    width: i32,
    height: i32,
    widgets: Vec<Widget>,
    current_species: Species,
}


impl crate::Game for MyGame {
    fn init() -> Self {
        let width = 256;
        let height = 256;

        let mut universe = Universe::new(width, height);
        
         
        let mut x = 5;
        while x <= (width - 5) {
            universe.paint(x, height - 40 + 5 * (f32::sin(x as f32 / 20.0)) as i32, (api::frnd() * 6.0) as i32 + 10,  Species::Sand);
            x += 10;
        }

        let mut x = 40;
        while x <= (width - 5) {
            universe.paint(x, height / 2 + 20 * (f32::sin(x as f32/20.0)) as i32, 6,  Species::Seed);
            x += 50 + api::rnd_range(0, 10);
        }
        
        // Draw the menus
        let mut widgets = Vec::new();

        widgets.push(Widget::new(Species::Water, "Water".to_string(), 1, width as u32 + 2, 0, 2, false));
        widgets.push(Widget::new(Species::Fire, "Fire".to_string(), 2, width as u32 + 2, 8, 2, false));
        widgets.push(Widget::new(Species::Lava, "Lava".to_string(), 3, width as u32 + 2, 16, 2, false));
        widgets.push(Widget::new(Species::Seed, "Seed".to_string(), 4, width as u32 + 2, 24, 2, false));
        widgets.push(Widget::new(Species::Sand, "Sand".to_string(), 5, width as u32 + 2, 32, 2, false));
        widgets.push(Widget::new(Species::Plant, "Plant".to_string(), 6, width as u32 + 2, 40, 2, false));
        widgets.push(Widget::new(Species::Rocket, "Rocket".to_string(), 7, width as u32 + 2, 48, 2, false));
        widgets.push(Widget::new(Species::Oil, "Oil".to_string(), 8, width as u32 + 2,56, 2, false));
        widgets.push(Widget::new(Species::Acid, "Acid".to_string(), 9, width as u32 + 2,64, 2, false));
        widgets.push(Widget::new(Species::Stone, "Stone".to_string(), 10, width as u32 + 2, 72, 2, false));
        widgets.push(Widget::new(Species::Wood, "Wood".to_string(), 11, width as u32 + 2, 80, 2, false));
        widgets.push(Widget::new(Species::Mite, "Mite".to_string(), 12, width as u32 + 2, 88, 2, false));
        widgets.push(Widget::new(Species::Gas, "Gas".to_string(), 13, width as u32 + 2, 96, 2, false));
        widgets.push(Widget::new(Species::Ice, "Ice".to_string(), 14, width as u32 + 2, 104, 2, false));

        // Cloner
        // Dust
        // Fungus
        // Empty

        Self {
            universe: universe,
            width: width,
            height: height,
            widgets: widgets,
            current_species: Species::Sand,
        }
    }

    /// Handle all of your game state logic here
    fn update(&mut self) {
        for widget in self.widgets.iter_mut() {
            if widget.update( api::mouse_left_state(0), api::mouse_x(), api::mouse_y()) {
                self.current_species = widget.species;
            }
        }

        if api::mouse_left_state(0) {
            self.universe.paint(api::mouse_x() as i32, api::mouse_y() as i32, 10, self.current_species);
        }

        self.universe.tick();
    }
    
   /// Handle all of your rendering code here
    fn draw(&mut self) {
        api::cls(0);

        // Draw menu
        for widget in self.widgets.iter_mut() {
            widget.draw();
        }

        // Draw all cells !
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.universe.get_cell(x, y);
                let rgbcolor = species_to_rgb(cell.species, cell.ra, cell.rb);
                api::pset_rgba(x, y, (rgbcolor.r * 255.0) as u8, (rgbcolor.g * 255.0) as u8, (rgbcolor.b *255.) as u8, 0xFF);
            }
        }
        //api::debug_print(format!("cells {:?}", self.universe.cells.len()).as_str());
    }



}