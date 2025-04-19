use std::collections::{BTreeMap, HashMap, LinkedList};
use std::time::Instant;

fn benchmark<T>(name: &str, collection: T)
where
    T: Fn(),
{
    let start = Instant::now();
    collection();
    let duration = start.elapsed();
    println!("Time taken for {}: {:?}", name, duration);
}

fn main() {
    benchmark("Vector", || {
        let mut nums = vec![];
        (1..=100000).for_each(|x| nums.push(x));
    });

    benchmark("LinkedList", || {
        let mut nums = LinkedList::new();
        (1..=100000).for_each(|x| nums.push_back(x));
    });

    benchmark("HashMap", || {
        let mut nums = HashMap::new();
        (1..=100000).for_each(|x| {
            nums.insert(x, 0);
        });
    });

    benchmark("BTreeMap", || {
        let mut nums = BTreeMap::new();
        (1..=100000).for_each(|x| {
            nums.insert(x, 0);
        });
    });
}
