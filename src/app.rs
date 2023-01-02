
use std::{thread};
use graphics::{glyph_cache::rusttype::GlyphCache, Graphics, CharacterCache, Context};
use opengl_graphics::{GlGraphics};
use piston::{RenderArgs, UpdateArgs, Key};
use crate::{constants::BLACK, sharewrapper::ShareWrapper, shared::{Status}, sort};
use crate::constants::WHITE;
use crate::constants::GREEN;
use crate::constants::WIDTH;
use crate::constants::HEIGHT;

pub struct App {
    pub sw: ShareWrapper,
} 

impl App {
    pub fn render<G: Graphics, C>(&mut self, glyphs: &mut C, c: &Context, g: &mut G) 
    where C: CharacterCache<Texture = G::Texture>, {
        use graphics::*;
            clear(BLACK, g);
            let sw = &self.sw.arc.lock().unwrap();
            let len = sw.get_len();
            let vec = &sw.vec;
            let delta_width: f64 = (WIDTH as f64/ len as f64).into();
            let delta_height: f64 = (HEIGHT as f64/ len as f64).into();

            for i in 0..len {
                let current_color = sw.get_current_idx(); 
                let curr = vec[i] as f64;
                let color = match current_color {
                    Some(idx) => if i == idx {GREEN} else {WHITE},
                    None => WHITE
                };
                let x: f64 = i as f64 * delta_width;
                let y: f64 = curr * delta_height;
                text(WHITE, 12, "This is a test", glyphs, c.transform, g).expect("Failed rendering text");
                rectangle(color, [x, HEIGHT.into(), delta_width, -(y + delta_height)], c.transform, g);
            }
    }

    pub fn update(&mut self, args: &UpdateArgs) {

    }

    pub fn press(&mut self, key: Key) {
        self.input(key);
    }

    fn input(&mut self, key: Key) {
        //Match sort commands
        let delta = 100;
        let mut sw = self.sw.clone();
        if key == Key::D0 {
            sw.pause_unpause();
        } else if key == Key::Up {
            sw.increase_size(delta);
        } else if key == Key::Down {
            sw.decrease_size(delta);
        }
        let status = sw.get_status();
        if status == Status::NotSorting {
            let _thread = thread::spawn(move || {
                match key {
                    Key::Space => sw.shuffle(),
                    //Process sorts
                    _ => App::sort_input(sw, key),
                };
            });
        }
        
    }

    fn sort_input(mut sw: ShareWrapper, key: Key) {
        let len = sw.get_len();
        match key { 
            Key::D1 => sort::bubblesort(&mut sw),
            Key::D2 => sort::selectionsort(&mut sw),
            Key::D3 => sort::mergesort(&mut sw, 0, len - 1),
            Key::D4 => sort::quicksort(&mut sw, 0, (len - 1) as isize),
            _ => {},
        }
        sw.set_status(Status::NotSorting);
    }

}