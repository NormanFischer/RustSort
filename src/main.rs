extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod sort;

use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, Key, PressEvent};
use rand::thread_rng;
use rand::prelude::SliceRandom;
use sort::input_sort;


const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

const WIDTH: u32 = 720;
const HEIGHT: u32 = 480;

pub struct App {
    gl: GlGraphics,
    vec: Arc<Mutex<Vec<u32>>>,
    sorting: bool,
    accesses: u32,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
            let len = self.vec.lock().unwrap().len();
            let delta_width: f64 = (WIDTH as f64/ len as f64).into();
            let delta_height: f64 = (HEIGHT as f64/ len as f64).into();
            for i in 0..len {
                let curr = self.vec.lock().unwrap()[i] as f64;
                // rect: x1, y1, x2, y2
                let x: f64 = i as f64 * delta_width;
                let y: f64 = curr * delta_height;
                rectangle(WHITE, [x, HEIGHT.into(), delta_width, -(y + delta_height)], c.transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {

    }

    fn press(&mut self, key: Key) {
        let guard = self.vec.lock().unwrap();
        let n = guard.len();
        drop(guard);
        let thread_arc = self.vec.clone();
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
            self.vec.lock().unwrap().shuffle(&mut thread_rng());
            println!("{:?}", self.vec);
        }
    }
}

fn is_sorted(vec: Vec<u32>) -> bool {
    for i in 0..(vec.len() - 2) {
        if vec[i] > vec[i + 1] {
            return false;
        }
    }
    return true;
}



fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
  
    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Sorting Algorithms", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        vec: Arc::new(Mutex::new((1..10001).collect())),
        sorting: false,
        accesses: 0,
    };

    println!("{:?}", app.vec);

    let mut events = Events::new(EventSettings::new());
    
    while let Some(e) = events.next(&mut window) {

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.press(key);
        }
            
    }
}
