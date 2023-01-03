use std::fmt;


//Collection of data that is shared by the program threads
#[derive(PartialEq, Copy, Clone)]
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

#[derive(Copy, Clone)]
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
    pub current_sort: Sort,
    pub comparisons: u64,
}

impl Shared{}