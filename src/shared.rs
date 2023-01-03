use std::fmt;

use rand::{seq::SliceRandom, thread_rng};


//Collection of data that is shared by the program threads
#[derive(PartialEq)]
pub enum Status {
    Sorting,
    Paused,
    NotSorting,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Status::Sorting => write!(f, "Sorting"),
           Status::Paused => write!(f, "Paused"),
           Status::NotSorting => write!(f, "Not Sorting"),
       }
    }
}

pub enum Sort {
    Bubblesort,
    Insertionsort,
    Selectionsort,
    Mergesort,
    Quicksort,
    None,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Sort::Bubblesort => write!(f, "Bubble sort"),
            Sort::Insertionsort => write!(f, "Insertion sort"),
            Sort::Selectionsort => write!(f, "Selection sort"),
            Sort::Mergesort => write!(f, "Merge sort"),
            Sort::Quicksort => write!(f, "Quick sort"),
            Sort::None => write!(f, "None"),
        }
    }
}
pub struct Shared{
    pub vec: Vec<u32>,
    pub status: Status,
    pub tickrate: u64,
    pub current_idx: Option<usize>,
    pub current_sort: Sort
}

impl Shared{

    pub fn get_vec(&self) -> Vec<u32> {
        return self.vec.clone();
    }

    pub fn resize(&mut self, new_size: u32) {
        self.vec = (1..=new_size).collect();
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

    pub fn get_current_sort(&self) -> Sort {
        match self.current_sort {
            Sort::Bubblesort => Sort::Bubblesort,
            Sort::Insertionsort => Sort::Insertionsort,
            Sort::Selectionsort => Sort::Selectionsort,
            Sort::Mergesort => Sort::Mergesort,
            Sort::Quicksort => Sort::Quicksort,
            Sort::None => Sort::None,
        }
    }

    pub fn set_current_sort(&mut self, sort: Sort) {
        self.current_sort = sort; 
    }
}