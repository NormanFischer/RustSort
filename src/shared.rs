
//Collection of data that is shared by the program threads
#[derive(PartialEq)]
pub enum Status {
    Sorting,
    Paused
}
pub struct Shared{
    pub vec: Vec<u32>,
    pub status: Status,
    pub tickrate: u64,
}

impl Shared{}