extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate find_folder;

mod sort;
mod shared;
mod sharewrapper;
mod app;
mod constants;

use std::sync::{Mutex, Arc};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, PressEvent};
use constants::WIDTH;
use constants::HEIGHT;

use crate::shared::{Status, Shared};
use crate::sharewrapper::ShareWrapper;

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
    let vector_size = 101;
  
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
        sw: ShareWrapper {
                arc: Arc::new(Mutex::new(
                              Shared { vec: (1..vector_size).collect(), status: Status::Paused, tickrate: 1, current_idx: None}))},
    };

    println!("{:?}", app.sw.arc.lock().unwrap().vec);

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
