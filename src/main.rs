extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod sort;

use core::time;
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{EventLoop, Button, Key, PressEvent};
use rand::thread_rng;
use rand::prelude::SliceRandom;


const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

const WIDTH: u32 = 720;
const HEIGHT: u32 = 480;

pub struct App {
    gl: GlGraphics,
    vec: Arc<Mutex<Vec<u32>>>,
    sorting: bool,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
            let len = self.vec.lock().unwrap().len();
            for i in 0..len {
                let curr = self.vec.lock().unwrap()[i] as f64;
                let delta_width: f64 = (WIDTH as f64/ len as f64).into();
                let delta_height: f64 = (HEIGHT as f64/ len as f64).into();
                // rect: x1, y1, x2, y2
                let x: f64 = i as f64 * delta_width;
                let y: f64 = curr * delta_height;
                rectangle(WHITE, [x, y + delta_height, delta_width, HEIGHT.into()], c.transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {

    }
}



fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
  
    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Sorting algorithms", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        vec: Arc::new(Mutex::new((1..101).collect())),
        sorting: false,
    };

    
    app.vec.lock().unwrap().shuffle(&mut thread_rng());
    println!("{:?}", app.vec);

    let mut events = Events::new(EventSettings::new()).ups(10);
    
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::C {
                println!("Sorting");
                let n = app.vec.lock().unwrap().len();
                let thread_arc = app.vec.clone();
                let thread = thread::spawn(move || {
                    for i in 0..n-1 {
                        for j in 0..n-i-1 {
                            if let Ok(mut vec) = thread_arc.lock() {
                                if vec[j] > vec[j+1] {
                                    vec.swap(j, j+1);
                                }
                                drop(vec);
                                thread::sleep(Duration::from_micros(10));
                            }
                        }   
                    }
                });
            }
        }
    }
}
