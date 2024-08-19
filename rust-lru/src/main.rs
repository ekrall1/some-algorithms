use std::collections::{HashMap, VecDeque};

pub struct LRU {
    capacity: i32,
    key_val_map: HashMap<String, f64>,
    deq: VecDeque<String>,
}

impl LRU {
    pub fn new(c: i32) -> Self {
        Self {
            capacity: c,
            key_val_map: HashMap::new(),
            deq: VecDeque::new(),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<f64> {
        match self.key_val_map.contains_key(key) {
            true => {
                self._remove_key_from_deq(String::from(key));
                self.deq.push_front(String::from(key));
                Some(self.key_val_map[key])
            }
            false => None,
        }
    }

    pub fn put(&mut self, key: &str, value: f64) {
        match self.key_val_map.contains_key(key) {
            true => {
                self._remove_key_from_deq(String::from(key));
            }
            false => {
                self._remove_oldest_from_map();
            }
        }

        self.key_val_map.insert(String::from(key), value);
        self.deq.push_front(String::from(key));
    }

    fn _remove_key_from_deq(&mut self, key: String) {
        let idx = self.deq.iter().position(|x| x == &key);
        self.deq.remove(idx.unwrap());
    }

    fn _remove_oldest_from_map(&mut self) {
        if self.deq.len() == self.capacity.try_into().unwrap() {
            let oldest = &self.deq.pop_back().unwrap();
            self.key_val_map.remove(oldest);
        }
    }
}

fn main() {
    let mut cache = LRU::new(4);

    let result_a = cache.get("Bob");
    assert_eq!(result_a, None);

    cache.put("Alice", 1.0);
    cache.put("Bob", 2.0);
    cache.put("Charlie", 3.0);
    cache.put("Dio", 4.0);

    let result_b = cache.get("Charlie");
    assert_eq!(result_b, Some(3.0));

    cache.put("Ed", 5.0);

    let result_c = cache.get("Ed");
    assert_eq!(result_c, Some(5.0));

    let result_d = cache.get("Alice");
    assert_eq!(result_d, None);

    println!("{:?}", "Success!");
}
