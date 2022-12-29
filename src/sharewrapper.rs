use std::{thread, time::Duration};



//Shared resources for application
pub struct ShareWrapper{
    pub vec: Vec<u32>,
    pub sorting: bool,
}

impl ShareWrapper{}