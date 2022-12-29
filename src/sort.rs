use std::{thread, sync::{Arc, Mutex}, time::Duration};
use owning_ref::{ArcRef, MutexGuardRef};
use piston::Key;

pub fn input_sort(thread_arc: Arc<Mutex<Vec<u32>>>, key: Key, n: usize) {
    let rc = thread_arc.into();
    match key {
        //Match sort commands
        Key::D1 => bubblesort(rc),
        Key::D2 => selectionsort(rc),
        Key::D3 => mergesort(&rc, 0, n - 1),
        _ => println!("Unimplemented"),
    };
}


fn bubblesort(rc: ArcRef<Mutex<Vec<u32>>>) {
    let n = rc.lock().unwrap().len();
    for i in 0..n-1 {
        for j in 0..n-i-1 {
                if let Ok(mut vec) = rc.lock() {
                    if vec[j] > vec[j+1] {
                        vec.swap(j, j+1);
                    }
                }
                thread::sleep(Duration::from_micros(1));
            }
    }
    println!("Done sorting"); 
}

fn selectionsort(rc: ArcRef<Mutex<Vec<u32>>>) {
    let n = rc.lock().unwrap().len();
    for i in 0..n-1 {
        let mut minindex = i;
        for j in (i+1)..n {
            if let Ok(vec) = rc.lock() {
                if vec[j] < vec[minindex] {
                    minindex = j;
                }
                
            }
            thread::sleep(Duration::from_micros(1));
        }   
        if let Ok(mut vec) = rc.lock() {
            vec.swap(i, minindex);
        }
    }
}


fn mergesort<'a>(rc: &'a ArcRef<Mutex<Vec<u32>>>, left: usize, right: usize) {
    if left < right {
        println!("Merging!");
        let m = (left + right) / 2;

        mergesort(rc, left, m);
        mergesort(rc, m + 1, right);

        //Create left and right subarrays
        let mut left_vec = Vec::new();
        let mut right_vec = Vec::new();

        let left_len = m - left + 1;
        let right_len = right - m;

        for i in 0..left_len {
            if let Ok(vec) = rc.lock() {
                left_vec.push(vec[left + i]);
            }
        }
        for i in 0..right_len {
            if let Ok(vec) = rc.lock() {
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
                    vec[k] = left_vec[i];
                }
                thread::sleep(Duration::from_micros(1));
                i = i + 1;
            } else {
                if let Ok(mut vec) = rc.lock() {
                    vec[k] = right_vec[j];
                }
                thread::sleep(Duration::from_micros(1));
                j = j + 1;
            }
            k = k + 1;
        }

        while i < left_len {
            if let Ok(mut vec) = rc.lock() {
                vec[k] = left_vec[i];
            }
            thread::sleep(Duration::from_micros(1));
            i = i + 1;
            k = k + 1;
        }
        while j < right_len {
            if let Ok(mut vec) = rc.lock() {
                vec[k] = right_vec[j];
            }
            thread::sleep(Duration::from_micros(1));
            j = j + 1;
            k = k + 1;
        }
    }
}

