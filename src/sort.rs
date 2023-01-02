use std::{thread, sync::{Arc, Mutex}, time::Duration};
use crate::{shared::{Status, Shared}, sharewrapper::ShareWrapper};


fn tick_checker(sw: &mut ShareWrapper) {
    //Extract status from sharewrapper
    let status = sw.get_status();
    let mut status_val = match status {
        Status::Paused => Status::Paused,
        Status::Sorting => Status::Sorting,
        Status::NotSorting => Status::NotSorting,
    };

    //If status is paused, loop until unpaused, keep extracting status
    while status_val == Status::Paused {
        let status = sw.get_status();
        status_val = match status {
        Status::Paused => Status::Paused,
        Status::Sorting => Status::Sorting,
        Status::NotSorting => Status::NotSorting,
        };
    }

    //Tick
    thread::sleep(Duration::from_micros(sw.get_tickrate()));
}


pub fn bubblesort(sw: &mut ShareWrapper) {
    sw.set_status(Status::Sorting);
    let n = sw.get_len();
    for i in 0..n-1 {
        for j in 0..n-i-1 {
            sw.set_current_idx(Some(j));
                if let Ok(mut guard) = sw.arc.lock() {
                    let vec = &mut guard.vec;
                    if vec[j] > vec[j+1] {
                        vec.swap(j, j+1);
                    }
                }
                tick_checker(sw);
            }
        }
    sw.set_current_idx(None);
}


pub fn selectionsort(sw: &mut ShareWrapper) {
    sw.set_status(Status::Sorting);
    let n = sw.get_len();
    for i in 0..n-1 {
        let mut minindex = i;
        for j in (i+1)..n {
            sw.set_current_idx(Some(j));
            if let Ok(mut guard) = sw.arc.lock() {
                let vec = &mut guard.vec; 
                if vec[j] < vec[minindex] {
                    minindex = j;
                }
            }
            tick_checker(sw);
        }   
        if let Ok(mut guard) = sw.arc.lock() {
            let vec = &mut guard.vec;
            vec.swap(i, minindex);
        }
    }
}


pub fn mergesort(sw: &mut ShareWrapper, left: usize, right: usize) {
    sw.set_status(Status::Sorting);
    if left < right {
        let m = (left + right) / 2;

        mergesort(sw, left, m);
        tick_checker(sw);
        mergesort(sw, m + 1, right);
        tick_checker(sw);
        //Create left and right subarrays
        let mut left_vec = Vec::new();
        let mut right_vec = Vec::new();

        let left_len = m - left + 1;
        let right_len = right - m;

        for i in 0..left_len {
            if let Ok(mut guard) = sw.arc.lock() {
                let vec = &mut guard.vec;
                left_vec.push(vec[left + i]);
            }
        }
        for i in 0..right_len {
            if let Ok(mut guard) = sw.arc.lock() {
                let vec = &mut guard.vec;
                right_vec.push(vec[m + 1 + i]);
            }
        }

        //Merge process
        let mut i = 0;
        let mut j = 0;
        let mut k = left;

        while i < left_len && j < right_len {
            if left_vec[i] <= right_vec[j] {
                sw.set_current_idx(Some(i));
                if let Ok(mut guard) = sw.arc.lock() {
                    let vec = &mut guard.vec;
                    vec[k] = left_vec[i];
                }
                tick_checker(sw);
                i = i + 1;
            } else {
                sw.set_current_idx(Some(j));
                if let Ok(mut guard) = sw.arc.lock() {
                    let vec = &mut guard.vec;
                    vec[k] = right_vec[j];
                }
                tick_checker(sw);
                j = j + 1;
            }
            k = k + 1;
        }

        while i < left_len {
            sw.set_current_idx(Some(i));
            if let Ok(mut guard) = sw.arc.lock() {
                let vec = &mut guard.vec;
                vec[k] = left_vec[i];
            }
            tick_checker(sw);
            i = i + 1;
            k = k + 1;
        }
        while j < right_len {
            sw.set_current_idx(Some(j));
            if let Ok(mut guard) = sw.arc.lock() {
                let vec = &mut guard.vec;
                vec[k] = right_vec[j];
            }
            tick_checker(sw);
            j = j + 1;
            k = k + 1;
        }
    }
}

pub fn quicksort<'a>(sw: &'a mut ShareWrapper, left: isize, right: isize) {
    sw.set_status(Status::Sorting);
    let pivotidx: isize;
    if left < right {
        pivotidx = partition(sw, left, right);
        if pivotidx == -100 {
            return;
        }
        quicksort(sw, left, pivotidx - 1);
        if pivotidx == -100 {
            return;
        }
        quicksort(sw, pivotidx + 1, right);
    }
}

fn partition<'a>(sw: &'a mut ShareWrapper, left: isize, right: isize) -> isize {
    let pivot: u32 = sw.get_idx(right as usize);
    let mut t = left;
    for i in left..right {
        if let Ok(mut guard) = sw.arc.lock() {
            let vec = &mut guard.vec;
            if vec[i as usize] <= pivot {
                vec.swap(t as usize, i as usize);
                t = t + 1;
            }
        }
        tick_checker(sw);
    }
    if let Ok(mut guard) = sw.arc.lock() {
        let vec = &mut guard.vec;
        vec.swap(t as usize, right as usize);
    }
    
    
    tick_checker(sw);
    return t;
}

