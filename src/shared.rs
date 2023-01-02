use rand::{seq::SliceRandom, thread_rng};


//Collection of data that is shared by the program threads
#[derive(PartialEq)]
pub enum Status {
    Sorting,
    Paused,
    NotSorting,
}

pub enum Sort {
    Bubblesort,
    Insertionsort,
    Selectionsort,
    Mergesort,
    Quicksrot,
}
pub struct Shared{
    pub vec: Vec<u32>,
    pub status: Status,
    pub tickrate: u64,
    pub current_idx: Option<usize>,
}

impl Shared{
    pub fn resize(&mut self, new_size: u32) {
        self.vec = (1..new_size).collect();
    }

    pub fn get_len(&self) -> usize {
        return self.vec.len();
    }

    pub fn get_idx(&self, idx: usize) -> u32 {
        self.vec[idx]
    }

    pub fn shuffle(&mut self) {
        self.vec.shuffle(&mut thread_rng());
    }

    pub fn get_status(&self) -> Status {
        match self.status {
            Status::Sorting => Status::Sorting,
            Status::Paused => Status::Paused,
            Status::NotSorting => Status::NotSorting,
        }
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn get_tickrate(&self) -> u64 {
        return self.tickrate;
    }

    pub fn get_current_idx(&self) -> Option<usize> {
        return self.current_idx;
    }

    pub fn set_current_idx(&mut self, idx: Option<usize>) {
        self.current_idx = idx;
    }
}