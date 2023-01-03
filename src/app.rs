
use std::{thread, time::Duration};
use graphics::{glyph_cache::rusttype::GlyphCache, Graphics, CharacterCache, Context};
use opengl_graphics::{GlGraphics};
use piston::{RenderArgs, UpdateArgs, Key};
use crate::{constants::BLACK, sharewrapper::ShareWrapper, shared::{Status, Sort}, sort};
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
        let sharewrapper = &mut self.sw;
        let current_color = sharewrapper.get_current_idx();
        let len = sharewrapper.get_len();
        let status = sharewrapper.get_status();
        let current_sort = &sharewrapper.get_current_sort();
        let tickrate = &sharewrapper.get_tickrate();
        let comparisons = &sharewrapper.get_comparsions();


        let status_txt: &str = &format!("Status: {}", status);
        let sort_txt: &str = &format!("Sort: {}", current_sort);
        let length_txt: &str = &format!("Array length: {}", len);
        let speed_txt: &str = &format!("Speed: {}", tickrate);
        let comparisons_txt: &str = &format!("Comparisons {}", comparisons);

        let font_size: f64 = 24.0;

        text(WHITE, font_size as u32, status_txt, glyphs, c.transform.trans(0.0, font_size * 1.0).zoom(0.5), g).expect("Failed rendering text");
        text(WHITE, font_size as u32, sort_txt, glyphs, c.transform.trans(0.0, font_size * 2.0).zoom(0.5), g).expect("Failed rendering text");
        text(WHITE, font_size as u32, length_txt, glyphs, c.transform.trans(0.0, font_size * 3.0).zoom(0.5), g).expect("Failed rendering text");
        text(WHITE, font_size as u32, speed_txt, glyphs, c.transform.trans(0.0, font_size * 4.0).zoom(0.5), g).expect("Failed rendering text");
        text(WHITE, font_size as u32, comparisons_txt, glyphs, c.transform.trans(0.0, font_size * 5.0).zoom(0.5), g).expect("Failed rendering text");

        let delta_width: f64 = (WIDTH as f64/ len as f64).into();
        let delta_height: f64 = (HEIGHT as f64/ len as f64).into();

        if let Ok(guard) = sharewrapper.arc.lock() {
            let vec = &guard.vec;
            for i in 0..len {
                let curr = vec[i] as f64;
                let color = match current_color {
                    Some(idx) => if i == idx {GREEN} else {WHITE},
                    None => WHITE
                };
                let x: f64 = i as f64 * delta_width;
                let y: f64 = curr * delta_height;
                rectangle(color, [x, HEIGHT.into(), delta_width, -(y + delta_height)], c.transform, g);
            }
        }
            
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        //thread::sleep(Duration::from_micros(self.sw.get_tickrate()));
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
        sw.set_status(Status::Sorting);
        match key { 
            Key::D1 => sort::bubblesort(&mut sw),
            Key::D2 => sort::selectionsort(&mut sw),
            Key::D3 => sort::mergesort(&mut sw, 0, len - 1),
            Key::D4 => sort::quicksort(&mut sw, 0, (len - 1) as isize),
            _ => {},
        }
        sw.set_current_sort(Sort::None);
        sw.set_status(Status::NotSorting);
        sw.reset_comparions();
    }

}