use std::{thread, time::Duration};

// Random function for threads test
fn threads_test() {
    let limit: i32 = 5;

    let handle = thread::spawn(move || {
       for n in 1..=limit {
           println!("Spawned, {}", n * n);
           thread::sleep(Duration::from_millis(1))
       }
    });

    for n in 1..=limit {
        println!("Main, {}", n * n);
        thread::sleep(Duration::from_millis(1))
    }

    handle.join().unwrap();
}