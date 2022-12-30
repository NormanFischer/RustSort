use std::{thread, sync::{Arc, Mutex}, time::Duration};
use crate::{sharewrapper::ShareWrapper};


fn tick_checker(rc: &Arc<Mutex<ShareWrapper>>) -> bool{
    thread::sleep(Duration::from_micros(1));
    if let Ok(guard) = rc.lock() {
        let sorting = guard.sorting;
        if sorting == false {
            return false;
        } else {
            let paused = guard.paused;
            if paused == true {
                println!("Paused");
            }
            return true;
        }
    }
    return false;
}


pub fn bubblesort(rc: &Arc<Mutex<ShareWrapper>>) {
    rc.lock().unwrap().sorting = true;
    let n = &rc.lock().unwrap().vec.len();
    for i in 0..n-1 {
        for j in 0..n-i-1 {
                if let Ok(mut vec) = rc.lock() {
                    let vec = &mut vec.vec;
                    if vec[j] > vec[j+1] {
                        vec.swap(j, j+1);
                    }
                }
                if tick_checker(rc) == false {
                    return; 
                } 
            }
    }
}



pub fn selectionsort(rc: &Arc<Mutex<ShareWrapper>>) {
    rc.lock().unwrap().sorting = true;
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
            if tick_checker(rc) == false {
                return; 
            } 
        }   
        if let Ok(mut vec) = rc.lock() {
            let vec = &mut vec.vec;
            vec.swap(i, minindex);
        }
    }
}


pub fn mergesort<'a>(rc: &'a Arc<Mutex<ShareWrapper>>, left: usize, right: usize) {
    rc.lock().unwrap().sorting = true;
    if left < right {
        let m = (left + right) / 2;

        mergesort(rc, left, m);
        if tick_checker(rc) == false {
            return; 
        } 
        mergesort(rc, m + 1, right);
        if tick_checker(rc) == false {
            return; 
        } 
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
                if tick_checker(rc) == false {
                    return; 
                } 
                i = i + 1;
            } else {
                if let Ok(mut vec) = rc.lock() {
                    let vec = &mut vec.vec;
                    vec[k] = right_vec[j];
                }
                if tick_checker(rc) == false {
                    return; 
                } 
                j = j + 1;
            }
            k = k + 1;
        }

        while i < left_len {
            if let Ok(mut vec) = rc.lock() {
                let vec = &mut vec.vec;
                vec[k] = left_vec[i];
            }
            if tick_checker(rc) == false {
                return; 
            } 
            i = i + 1;
            k = k + 1;
        }
        while j < right_len {
            if let Ok(mut vec) = rc.lock() {
                let vec = &mut vec.vec;
                vec[k] = right_vec[j];
            }
            if tick_checker(rc) == false {
                return; 
            } 
            j = j + 1;
            k = k + 1;
        }
    }
}

pub fn quicksort<'a>(rc: &'a Arc<Mutex<ShareWrapper>>, left: isize, right: isize) {
    rc.lock().unwrap().sorting = true;
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

fn partition<'a>(rc: &'a Arc<Mutex<ShareWrapper>>, left: isize, right: isize) -> isize {
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
        if tick_checker(rc) == false {
            return -100; 
        } 
    }
    if let Ok(mut vec) = rc.lock() {
        let vec = &mut vec.vec;
        vec.swap(t as usize, right as usize);
    }
    if tick_checker(rc) == false {
        return -100; 
    } 
    return t;
}

