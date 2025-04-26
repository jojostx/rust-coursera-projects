use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let data = vec![1, 2, 3];
    let data_mx = Arc::new(Mutex::new(data));
    let mut handles = vec![];

    for i in 0..3 {
        let dlock = data_mx.clone();

        handles.push(thread::spawn(move || {
            let mut dlock = dlock.lock().unwrap();
            dlock[i] += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", data_mx.lock().unwrap());
}
