extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::Size;
use piston::EventLoop;
use rand::thread_rng;
use rand::prelude::SliceRandom;


const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

const WIDTH: u32 = 720;
const HEIGHT: u32 = 480;

pub struct App {
    gl: GlGraphics,
    vec: Vec<u32>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
            for i in 0..self.vec.len() {
                let mut curr = self.vec[i] as f64;
                let delta_width: f64 = (WIDTH as f64/ self.vec.len() as f64).into();
                let delta_height: f64 = (HEIGHT as f64/ self.vec.len() as f64).into();
                // rect: x1, y1, x2, y2
                let mut x: f64 = i as f64 * delta_width;
                let mut y: f64 = curr * delta_height;

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
        .resizable(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        vec: (1..101).collect(),
    };
    
    self.vec.shuffle(&mut thread_rng());

    let mut events = Events::new(EventSettings::new()).ups(10);
    println!("{:?}", app.vec);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
