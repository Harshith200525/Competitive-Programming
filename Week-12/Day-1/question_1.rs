// Question 1: LFU Cache
// How to design a cache with the least frequently used (LFU) eviction policy?

use std::collections::{HashMap, HashSet};

struct LFUCache {
    capacity: usize,
    min_freq: i32,
    key_to_val: HashMap<i32, i32>,
    key_to_freq: HashMap<i32, i32>,
    freq_to_keys: HashMap<i32, HashSet<i32>>,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            capacity: capacity as usize,
            min_freq: 0,
            key_to_val: HashMap::new(),
            key_to_freq: HashMap::new(),
            freq_to_keys: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&value) = self.key_to_val.get(&key) {
            self.update_freq(key);
            return value;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        if self.key_to_val.contains_key(&key) {
            self.key_to_val.insert(key, value);
            self.update_freq(key);
        } else {
            if self.key_to_val.len() == self.capacity {
                if let Some(&evict) = self.freq_to_keys.get_mut(&self.min_freq).unwrap().iter().next() {
                    self.freq_to_keys.get_mut(&self.min_freq).unwrap().remove(&evict);
                    self.key_to_val.remove(&evict);
                    self.key_to_freq.remove(&evict);
                }
            }

            self.key_to_val.insert(key, value);
            self.key_to_freq.insert(key, 1);
            self.freq_to_keys.entry(1).or_insert_with(HashSet::new).insert(key);
            self.min_freq = 1;
        }
    }

    fn update_freq(&mut self, key: i32) {
        let freq = self.key_to_freq.get_mut(&key).unwrap();
        self.freq_to_keys.get_mut(freq).unwrap().remove(&key);
        if self.freq_to_keys.get(freq).unwrap().is_empty() {
            self.freq_to_keys.remove(freq);
            if *freq == self.min_freq {
                self.min_freq += 1;
            }
        }

        *freq += 1;
        self.freq_to_keys.entry(*freq).or_insert_with(HashSet::new).insert(key);
    }
}
