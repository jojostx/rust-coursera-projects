use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let data = vec![1, 2, 3];
    let data_mx = Arc::new(Mutex::new(data));

    for i in 0..3 {
        // Try to capture a mutable reference in multiple threads
        // This will fail to compile!
        let dlock = data_mx.clone();

        thread::spawn(move || {
            // dlock[i] += 1;
            let mut dlock = dlock.lock().unwrap();
            dlock[i] += 1;
        });
    }

    // No data race can occur, this will not compile.
}
