extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod sort;
mod sharewrapper;
mod app;

use std::sync::{Mutex, Arc};

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, PressEvent};
use sharewrapper::ShareWrapper;


const WIDTH: u32 = 720;
const HEIGHT: u32 = 480;

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
    let mut app = app::App {
        gl: GlGraphics::new(opengl),
        sw: Arc::new(Mutex::new(ShareWrapper { vec: (1..10001).collect(), sorting: false,})),
    };

    println!("{:?}", app.sw.lock().unwrap().vec);

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
