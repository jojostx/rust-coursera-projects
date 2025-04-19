use std::{
    collections::{BTreeMap, HashMap, LinkedList},
    thread::{self, JoinHandle},
    time::Instant,
};

fn main() {
    let start = Instant::now();

    // spawn multiple threads for insertions into collection types
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    let handle1 = thread::spawn(|| {
        let start = Instant::now();
        let mut nums = vec![];
        (1..=10000).for_each(|x| {
            nums.push(x);
        });
        let duration = start.elapsed();
        println!("Time taken for Vector: {:?}", duration);
    });

    // LinkedList
    let handle2 = thread::spawn(|| {
        let start = Instant::now();
        let mut nums = LinkedList::new();
        (1..=10000).for_each(|x| {
            nums.push_back(x);
        });
        let duration = start.elapsed();
        println!("Time taken for LinkedList: {:?}", duration);
    });

    // HashMap
    let handle3 = thread::spawn(|| {
        let start = Instant::now();
        let mut nums = HashMap::new();
        (1..=10000).for_each(|x| {
            nums.insert(x, 0);
        });
        let duration = start.elapsed();
        println!("Time taken for HashMap: {:?}", duration);
    });

    // BTreeMap
    let handle4 = thread::spawn(|| {
        let start = Instant::now();
        let mut nums = BTreeMap::new();
        (1..=10000).for_each(|x| {
            nums.insert(x, 0);
        });
        let duration = start.elapsed();
        println!("Time taken for BTreeMap: {:?}", duration);
    });

    handles.push(handle1);
    handles.push(handle2);
    handles.push(handle3);
    handles.push(handle4);

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
}
