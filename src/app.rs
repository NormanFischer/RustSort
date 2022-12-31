
use std::{thread};
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs, Key};
use crate::{constants::BLACK, sharewrapper::ShareWrapper, shared::{Status}, sort};
use crate::constants::WHITE;
use crate::constants::GREEN;
use crate::constants::WIDTH;
use crate::constants::HEIGHT;

pub struct App {
    pub gl: GlGraphics,
    pub sw: ShareWrapper,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
            let sw = &self.sw.arc.lock().unwrap();
            let vec = &sw.vec;
            let len = vec.len();
            let delta_width: f64 = (WIDTH as f64/ len as f64).into();
            let delta_height: f64 = (HEIGHT as f64/ len as f64).into();


            for i in 0..len {
                let current_color = sw.current_idx; 
                let curr = vec[i] as f64;
                let color = match current_color {
                    Some(idx) => if i == idx {GREEN} else {WHITE},
                    None => WHITE
                };
                // rect: x1, y1, x2, y2
                let x: f64 = i as f64 * delta_width;
                let y: f64 = curr * delta_height;
                rectangle(color, [x, HEIGHT.into(), delta_width, -(y + delta_height)], c.transform, gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {

    }

    pub fn press(&mut self, key: Key) {
        self.input(key);
    }

    fn input(&mut self, key: Key) {
        //Match sort commands
        let mut rc = self.sw.clone();
        if key == Key::D0 {
            self.sw.pause_unpause();
        }
        let status = self.sw.get_status();
        if status == Status::Paused {
            let _thread = thread::spawn(move || {
                match key {
                    Key::Space => rc.shuffle(),
                    //Process sorts
                    _ => App::sort_input(rc, key),
                };
            });
        }
        
    }

    fn sort_input(mut sw: ShareWrapper, key: Key) {
        let len = sw.get_len();
        match key { 
            Key::D1 => sort::bubblesort(&sw.arc),
            Key::D2 => sort::selectionsort(&sw.arc),
            Key::D3 => sort::mergesort(&sw.arc, 0, len - 1),
            Key::D4 => sort::quicksort(&sw.arc, 0, (len - 1) as isize),
            _ => {},
        }
        sw.arc.lock().unwrap().status = Status::Paused;
    }

}