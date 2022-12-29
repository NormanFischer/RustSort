
use std::{sync::{Arc, Mutex}, thread};
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs, Key};
use rand::{seq::SliceRandom, thread_rng};

use crate::{sharewrapper::ShareWrapper, sort::input_sort};

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

const WIDTH: u32 = 720;
const HEIGHT: u32 = 480;

pub struct App {
    pub gl: GlGraphics,
    pub sw: Arc<Mutex<ShareWrapper>>,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            let vec = &self.sw.lock().unwrap().vec;
            let len = vec.len();
            let delta_width: f64 = (WIDTH as f64/ len as f64).into();
            let delta_height: f64 = (HEIGHT as f64/ len as f64).into();

            for i in 0..len {
                let curr = vec[i] as f64;
                // rect: x1, y1, x2, y2
                let x: f64 = i as f64 * delta_width;
                let y: f64 = curr * delta_height;
                rectangle(WHITE, [x, HEIGHT.into(), delta_width, -(y + delta_height)], c.transform, gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {

    }

    pub fn press(&mut self, key: Key) {
        let vec = &mut self.sw.lock().unwrap().vec;
        let n = vec.len();

        let thread_arc = self.sw.clone();
        let thread = thread::spawn(move || {
            input_sort(thread_arc, key, n);
            /*
            if let Ok(vec) = thread_arc.lock() {
                if is_sorted(vec.to_vec()) {
                    println!("Sorted!");
                }
            }
            */
        });

        //Shuffle
        if key == Key::Space {
            vec.shuffle(&mut thread_rng());
            println!("{:?}", vec);
        }
    }
}