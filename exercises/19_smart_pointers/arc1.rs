// In this exercise, we are given a `Vec` of `u32` called `numbers` with values
// ranging from 0 to 99. We would like to use this set of numbers within 8
// different threads simultaneously. Each thread is going to get the sum of
// every eighth value with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, …
// The second thread (offset 1), will sum 1, 9, 17, …
// The third thread (offset 2), will sum 2, 10, 18, …
// …
// The eighth thread (offset 7), will sum 7, 15, 23, …
//
// Each thread should own a reference-counting pointer to the vector of
// numbers. But `Rc` isn't thread-safe. Therefore, we need to use `Arc`.
//
// Don't get distracted by how threads are spawned and joined. We will practice
// that later in the exercises about threads.

// Don't change the lines below.
#![forbid(unused_imports)]
use std::{sync::{Arc, Mutex}, thread};

fn main() {
  
    let mut my_vec = Vec::new();

    for i in 0..100 {
        my_vec.push(i);
    }

    let my_vec_shared = Arc::new(Mutex::new(my_vec));

    
    let mut handles = vec![];
    for i in 0..8 {

        let vec_copy = Arc::clone(&my_vec_shared);
        let handle = thread::spawn(move ||{
            let mut sum=0;
            for y in (i..100).step_by(8) {

                let vecc = vec_copy.lock().unwrap();
                sum +=  vecc[y]
            }

            println!("sum from {i}th thread is: {sum}")
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()
    }



}
