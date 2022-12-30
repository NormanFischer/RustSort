#[derive(PartialEq)]
pub enum Status {
    Sorting,
    Paused
}
pub struct ShareWrapper{
    pub vec: Vec<u32>,
    pub status: Status,
    pub tickrate: u64,
}

impl ShareWrapper{
}