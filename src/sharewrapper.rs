//Wrapper for shared resources
//Functions that allow safe access to shared resources

use std::{sync::{Arc, Mutex, MutexGuard}, usize};
use rand::{seq::SliceRandom, thread_rng};

use crate::shared::{Shared, Status, Sort};


pub struct ShareWrapper {
    pub arc: Arc<Mutex<Shared>>,
}

impl ShareWrapper {

    //Obtain current status value from shared
    pub fn get_status(&self) -> Status {
        if let Ok(guard) = self.arc.lock() {
            guard.status
        } else {
            panic!("get_status")
        }
    }

    pub fn set_status(&mut self, status: Status) {
        if let Ok(mut guard) = self.arc.lock() {
            guard.status = status;
        }
    }

    //Set pause status
    pub fn pause_unpause(&mut self) {
        if let Ok(mut guard) = self.arc.lock() {
            let status = &guard.status;
            match status {
                Status::Sorting => guard.status = Status::Paused,
                Status::Paused => guard.status = Status::Sorting,
                Status::NotSorting => {},
            }
        }
    }

    pub fn increase_size(&mut self, delta: u32) {
        if let Ok(mut guard) = self.arc.lock() {
            let status = &guard.status;
            let len = guard.vec.len() as u32;
            match status {
                Status::NotSorting => guard.vec = (1..=len+delta).collect(),
                _ => {},
            };
        }
    }

    pub fn decrease_size(&mut self, delta: u32) {
        if let Ok(mut guard) = self.arc.lock() {
            let status = &guard.status;
            let len = guard.vec.len() as u32;
            match status {
                Status::NotSorting => guard.vec = (1..=len-delta).collect(),
                _ => {},
            };
        }
    }

    fn resize(&mut self, mut guard: MutexGuard<Shared>, new_size: u32,) {
        guard.vec = (1..=new_size).collect();
    }

    pub fn get_idx(&self, idx:usize) -> u32 {
        if let Ok(guard) = self.arc.lock() {
            guard.vec[idx]
        } else {
            panic!("get_idx")
        }
    }

    //Shuffles shared vector
    pub fn shuffle(&mut self) {
        if let Ok(mut guard) = self.arc.lock() {
            guard.vec.shuffle(&mut thread_rng());
        }
    }

    //Get length of shared vector
    pub fn get_len(&self) -> usize {
        if let Ok(guard) = self.arc.lock() {
            guard.vec.len()
        } else {
            panic!();
        }
    }

    pub fn set_current_idx(&mut self, idx: Option<usize>) {
        if let Ok(mut guard) = self.arc.lock() {
            guard.current_idx = idx;
        }
    }

    pub fn get_current_idx(&self) -> Option<usize> {
        if let Ok(guard) = self.arc.lock() {
            guard.current_idx
        } else {
            panic!("Error returning idx")
        }
    }

    pub fn get_tickrate(&self) -> u64 {
        if let Ok(guard) = self.arc.lock() {
            guard.tickrate
        } else {
            panic!("Tickrate")
        }
    }

    pub fn get_current_sort(&self) -> Sort {
        if let Ok(guard) = self.arc.lock() {
            guard.current_sort
        } else {
            panic!("get_current_sort");
        }
    }

    pub fn set_current_sort(&mut self, sort: Sort) {
        if let Ok(mut guard) = self.arc.lock() {
            guard.current_sort = sort
        }
    }

    pub fn increment_comparions(&self) {
        if let Ok(mut guard) = self.arc.lock() {
            guard.comparisons = guard.comparisons + 1;
        }
    }

    pub fn get_comparsions(&self) -> u64 {
        if let Ok(guard) = self.arc.lock() {
            guard.comparisons
        } else {
            panic!("get_comparisons")
        }
    }

    pub fn reset_comparions(&self) {
        if let Ok(mut guard) = self.arc.lock() {
            guard.comparisons = 0;
        }
    }

    pub fn clone(&mut self) -> ShareWrapper {
        ShareWrapper { arc: self.arc.clone() }
    }
}