use std::collections::HashMap;
use std::collections::LinkedList;

fn main() {
    let mut lru = Lru::new(2);

    lru.put(1, 32);
    lru.put(2, 32);
    lru.put(2, 64);
    lru.put(3, 4);
    lru.put(3, 11);
    lru.get(3);

    print!("{:?}", lru);
}

#[derive(Debug)]
struct Lru {
    capacity: usize,
    data: HashMap<i32, i32>,
    usage: LinkedList<i32>,
}

impl Lru {
    fn new(capacity: usize) -> Self {
        Lru {
            capacity,
            data: HashMap::with_capacity(capacity),
            usage: LinkedList::new(),
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        // put the value in the data and add the key to the usage using the touch method.
        self.data.insert(key, value);
        self.touch(key);
    }

    fn get(&mut self, key: i32) -> Option<&i32> {
        // get the value at the key and touch the usage
        if self.data.contains_key(&key) {
            self.touch(key);
        }
        self.data.get(&key)
    }

    fn touch(&mut self, key: i32) {
        if self.data.len() > self.capacity {
            if let Some(value) = self.usage.pop_back() {
                self.data.remove(&value);
            }
        }

        // Manually remove the key from the usage list
        let mut new_usage = LinkedList::new();
        for &k in self.usage.iter() {
            if k != key {
                new_usage.push_back(k);
            }
        }

        self.usage = new_usage;
        self.usage.push_front(key);
    }
}
