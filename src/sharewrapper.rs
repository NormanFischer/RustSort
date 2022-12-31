//Wrapper for shared resources
//Functions that allow safe access to shared resources

use std::sync::{Arc, Mutex};
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

    pub fn clone(&mut self) -> ShareWrapper {
        ShareWrapper { arc: self.arc.clone() }
    }
}