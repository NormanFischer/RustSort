
use std::{sync::{Arc, Mutex, mpsc}, thread};
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs, Key};
use rand::{seq::SliceRandom, thread_rng};
use crate::{sharewrapper::{ShareWrapper, Status}, sort::{self}};
use crate::constants::BLACK;
use crate::constants::WHITE;
use crate::constants::WIDTH;
use crate::constants::HEIGHT;

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
        let rc = self.sw.clone();
        App::input(&rc, key)
    }

    fn input(rc: &Arc<Mutex<ShareWrapper>>, key: Key) {
        //Match sort commands
        let rc = rc.clone();
        if key == Key::D0 {
            App::pause_unpause(&rc);
        }
        let _thread = thread::spawn(move || {
            match key {
                Key::Space => Self::shuffle(&rc),
                //Process sorts
                _ => App::sort_input(&rc, key),
            };
        });
    }

    fn sort_input(rc: &Arc<Mutex<ShareWrapper>>, key: Key) {
        match key { 
            Key::D1 => sort::bubblesort(&rc),
            Key::D2 => sort::selectionsort(&rc),
            Key::D3 => sort::mergesort(&rc, 0, Self::get_len(&rc) - 1),
            Key::D4 => sort::quicksort(&rc, 0, (Self::get_len(&rc) - 1) as isize),
            _ => {},
        }
    }

    fn pause_unpause(rc: &Arc<Mutex<ShareWrapper>>) {
        if let Ok(mut guard) = rc.lock() {
            match guard.status {
                Status::Sorting => guard.status = Status::Paused,
                Status::Paused => guard.status = Status::Sorting,
            }
        }
    }

    fn shuffle(rc: &Arc<Mutex<ShareWrapper>>) {
        if let Ok(mut guard) = rc.lock() {
            let vec = &mut guard.vec;
            vec.shuffle(&mut thread_rng());
        }
    }

    fn get_len(rc: &Arc<Mutex<ShareWrapper>>) -> usize {
        if let Ok(guard) = rc.lock() {
            let vec = &guard.vec;
            return vec.len();
        } else {
            panic!();
        }
    }
}