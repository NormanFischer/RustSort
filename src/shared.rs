
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

impl Shared{}