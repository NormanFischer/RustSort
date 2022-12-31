use std::{thread, sync::{Arc, Mutex}, time::Duration};
use crate::shared::{Status, Shared};


fn tick_checker(rc: &Arc<Mutex<Shared>>) {
    //Extract status from sharewrapper
    let guard = rc.lock().unwrap();
    let status = &guard.status;
    let mut status_val = match status {
        Status::Paused => Status::Paused,
        Status::Sorting => Status::Sorting,
    };
    drop(guard);

    //If status is paused, loop until unpaused, keep extracting status
    while status_val == Status::Paused {
        let guard = rc.lock().unwrap();
        let status = &guard.status;
        status_val = match status {
        Status::Paused => Status::Paused,
        Status::Sorting => Status::Sorting,
        };
    
        drop(status);
    }

    //Tick
    thread::sleep(Duration::from_micros(1));
}


pub fn bubblesort(rc: &Arc<Mutex<Shared>>) {
    rc.lock().unwrap().status = Status::Sorting;
    let n = &rc.lock().unwrap().vec.len();
    for i in 0..n-1 {
        for j in 0..n-i-1 {
                if let Ok(mut vec) = rc.lock() {
                    let vec = &mut vec.vec;
                    if vec[j] > vec[j+1] {
                        vec.swap(j, j+1);
                    }
                }
                tick_checker(rc);
            }
    }
}



pub fn selectionsort(rc: &Arc<Mutex<Shared>>) {
    rc.lock().unwrap().status = Status::Sorting;
    let n = &rc.lock().unwrap().vec.len();
    for i in 0..n-1 {
        let mut minindex = i;
        for j in (i+1)..*n {
            if let Ok(mut vec) = rc.lock() {
                let vec = &mut vec.vec;
                if vec[j] < vec[minindex] {
                    minindex = j;
                }
                
            }
            tick_checker(rc);
        }   
        if let Ok(mut vec) = rc.lock() {
            let vec = &mut vec.vec;
            vec.swap(i, minindex);
        }
    }
}


pub fn mergesort<'a>(rc: &'a Arc<Mutex<Shared>>, left: usize, right: usize) {
    rc.lock().unwrap().status = Status::Sorting;
    if left < right {
        let m = (left + right) / 2;

        mergesort(rc, left, m);
        tick_checker(rc);
        mergesort(rc, m + 1, right);
        tick_checker(rc);
        //Create left and right subarrays
        let mut left_vec = Vec::new();
        let mut right_vec = Vec::new();

        let left_len = m - left + 1;
        let right_len = right - m;

        for i in 0..left_len {
            if let Ok(mut vec) = rc.lock() {
                let vec = &mut vec.vec;
                left_vec.push(vec[left + i]);
            }
        }
        for i in 0..right_len {
            if let Ok(mut vec) = rc.lock() {
                let vec = &mut vec.vec;
                right_vec.push(vec[m + 1 + i]);
            }
        }

        //Merge process
        let mut i = 0;
        let mut j = 0;
        let mut k = left;

        while i < left_len && j < right_len {
            if left_vec[i] <= right_vec[j] {
                if let Ok(mut vec) = rc.lock() {
                    let vec = &mut vec.vec;
                    vec[k] = left_vec[i];
                }
                tick_checker(rc);
                i = i + 1;
            } else {
                if let Ok(mut vec) = rc.lock() {
                    let vec = &mut vec.vec;
                    vec[k] = right_vec[j];
                }
                tick_checker(rc);
                j = j + 1;
            }
            k = k + 1;
        }

        while i < left_len {
            if let Ok(mut vec) = rc.lock() {
                let vec = &mut vec.vec;
                vec[k] = left_vec[i];
            }
            tick_checker(rc);
            i = i + 1;
            k = k + 1;
        }
        while j < right_len {
            if let Ok(mut vec) = rc.lock() {
                let vec = &mut vec.vec;
                vec[k] = right_vec[j];
            }
            tick_checker(rc);
            j = j + 1;
            k = k + 1;
        }
    }
}

pub fn quicksort<'a>(rc: &'a Arc<Mutex<Shared>>, left: isize, right: isize) {
    rc.lock().unwrap().status = Status::Sorting;
    let pivotidx: isize;
    if left < right {
        pivotidx = partition(rc, left, right);
        if pivotidx == -100 {
            return;
        }
        quicksort(rc, left, pivotidx - 1);
        if pivotidx == -100 {
            return;
        }
        quicksort(rc, pivotidx + 1, right);
    }
}

fn partition<'a>(rc: &'a Arc<Mutex<Shared>>, left: isize, right: isize) -> isize {
    let pivot: u32 = rc.lock().unwrap().vec[right as usize];
    let mut t = left;
    for i in left..right {
        if let Ok(mut vec) = rc.lock() {
            let vec = &mut vec.vec;
            if vec[i as usize] <= pivot {
                vec.swap(t as usize, i as usize);
                t = t + 1;
            }
        }
        tick_checker(rc);
    }
    if let Ok(mut vec) = rc.lock() {
        let vec = &mut vec.vec;
        vec.swap(t as usize, right as usize);
    }
    tick_checker(rc);
    return t;
}

