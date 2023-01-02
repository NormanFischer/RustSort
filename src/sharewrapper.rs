//Wrapper for shared resources
//Functions that allow safe access to shared resources

use std::{sync::{Arc, Mutex}, usize};
use crate::shared::{Shared, Status};


pub struct ShareWrapper {
    pub arc: Arc<Mutex<Shared>>,
}

impl ShareWrapper {
    //Obtain current status value from shared
    pub fn get_status(&mut self) -> Status {
        if let Ok(guard) = self.arc.lock() {
            guard.get_status()
        } else {
            panic!("get_status")
        }
    }

    pub fn set_status(&mut self, status: Status) {
        if let Ok(mut guard) = self.arc.lock() {
            guard.set_status(status);
        }
    }

    //Set pause status
    pub fn pause_unpause(&mut self) {
        if let Ok(mut guard) = self.arc.lock() {
            let status = guard.get_status();
            match status {
                Status::Sorting => guard.set_status(Status::Paused),
                Status::Paused => guard.set_status(Status::Sorting),
                Status::NotSorting => {},
            }
        }
    }

    pub fn increase_size(&mut self, delta: u32) {
        if let Ok(mut guard) = self.arc.lock() {
            let status = guard.get_status();
            let len = guard.get_len();
            match status {
                Status::NotSorting => guard.resize(len as u32 + delta),
                _ => {},
            };
        }
    }

    pub fn decrease_size(&mut self, delta: u32) {
        if let Ok(mut guard) = self.arc.lock() {
            let status = guard.get_status();
            let len = guard.get_len();
            match status {
                Status::NotSorting => guard.resize(len as u32 - delta),
                _ => {},
            };
        }
    }

    pub fn get_idx(&mut self, idx:usize) -> u32 {
        if let Ok(mut guard) = self.arc.lock() {
            guard.get_idx(idx)
        } else {
            panic!("get_idx")
        }
    }

    //Shuffles shared vector
    pub fn shuffle(&mut self) {
        if let Ok(mut guard) = self.arc.lock() {
            guard.shuffle();
        }
    }

    //Get length of shared vector
    pub fn get_len(&mut self) -> usize {
        if let Ok(guard) = self.arc.lock() {
            guard.get_len()
        } else {
            panic!();
        }
    }

    pub fn set_current_idx(&mut self, idx: Option<usize>) {
        if let Ok(mut guard) = self.arc.lock() {
            guard.set_current_idx(idx);
        }
    }

    pub fn get_current_idx(&self) -> usize {
        if let Ok(guard) = self.arc.lock() {
            return guard.get_current_idx().unwrap();
        } else {
            panic!("Error returning idx")
        }
    }

    pub fn get_tickrate(&self) -> u64 {
        if let Ok(guard) = self.arc.lock() {
            return guard.get_tickrate();
        } else {
            panic!("Tickrate")
        }
    }

    pub fn clone(&mut self) -> ShareWrapper {
        ShareWrapper { arc: self.arc.clone() }
    }
}