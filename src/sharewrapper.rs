//Wrapper for shared resources
//Functions that allow safe access to shared resources

use std::{sync::{Arc, Mutex}, usize};
use rand::{thread_rng, seq::SliceRandom};

use crate::shared::{Shared, Status};

pub struct ShareWrapper {
    pub arc: Arc<Mutex<Shared>>,
}

impl ShareWrapper {
    //Obtain current status value from shared
    pub fn get_status(&mut self) -> Status {
        if let Ok(guard) = self.arc.lock() {
            match guard.status {
                Status::Sorting => Status::Sorting,
                Status::Paused => Status::Paused,
                Status::NotSorting => Status::NotSorting,
            }
        } else {
            panic!("get_status")
        }
    }

    //Set pause status
    pub fn pause_unpause(&mut self) {
        if let Ok(mut guard) = self.arc.lock() {
            match guard.status {
                Status::Sorting => guard.status = Status::Paused,
                Status::Paused => guard.status = Status::Sorting,
                Status::NotSorting => {},
            }
        }
    }

    //Shuffles shared vector
    pub fn shuffle(&mut self) {
        if let Ok(mut guard) = self.arc.lock() {
            let vec = &mut guard.vec;
            vec.shuffle(&mut thread_rng());
        }
    }

    //Get length of shared vector
    pub fn get_len(&mut self) -> usize {
        if let Ok(guard) = self.arc.lock() {
            let vec = &guard.vec;
            return vec.len();
        } else {
            panic!();
        }
    }

    pub fn set_current(&mut self, idx: usize) {
        if let Ok(mut guard) = self.arc.lock() {
            guard.current_idx = Some(idx);
        }
    }

    pub fn get_current_idx(&self) -> usize {
        if let Ok(guard) = self.arc.lock() {
            return guard.current_idx.unwrap();
        } else {
            panic!("Error returning idx")
        }
    }

    pub fn clone(&mut self) -> ShareWrapper {
        ShareWrapper { arc: self.arc.clone() }
    }
}