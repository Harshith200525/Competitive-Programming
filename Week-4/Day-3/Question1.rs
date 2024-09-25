// 146. LRU Cache

// Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.

// Implement the LRUCache class:

// LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
// int get(int key) Return the value of the key if the key exists, otherwise return -1.
// void put(int key, int value) Update the value of the key if the key exists. Otherwise, add the key-value pair to the cache. If the number of keys exceeds the capacity from this operation, evict the least recently used key.
// The functions get and put must each run in O(1) average time complexity.

 

// Example 1:

// Input
// ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
// [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
// Output
// [null, null, null, 1, null, -1, null, -1, 3, 4]

// Explanation
// LRUCache lRUCache = new LRUCache(2);
// lRUCache.put(1, 1); // cache is {1=1}
// lRUCache.put(2, 2); // cache is {1=1, 2=2}
// lRUCache.get(1);    // return 1
// lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
// lRUCache.get(2);    // returns -1 (not found)
// lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
// lRUCache.get(1);    // return -1 (not found)
// lRUCache.get(3);    // return 3
// lRUCache.get(4);    // return 4
 

// Constraints:

// 1 <= capacity <= 3000
// 0 <= key <= 104
// 0 <= value <= 105
// At most 2 * 105 calls will be made to get and put.

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, Rc<RefCell<CacheNode>>>,
    head: Rc<RefCell<CacheNode>>,
    tail: Rc<RefCell<CacheNode>>,
}

struct CacheNode {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<CacheNode>>>,
    next: Option<Rc<RefCell<CacheNode>>>,
}

impl CacheNode {
    fn new(key: i32, value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            key,
            value,
            prev: None,
            next: None,
        }))
    }
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let head = CacheNode::new(-1, -1);
        let tail = CacheNode::new(-1, -1);
        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(head.clone());

        LRUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            head,
            tail,
        }
    }

    fn get(&self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key) {
            let node = node.clone();
            let mut node = node.borrow_mut();
            let value = node.value;
            self.remove(&node);
            self.add(&node);
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key) {
            let node = node.clone();
            let mut node = node.borrow_mut();
            node.value = value;
            self.remove(&node);
            self.add(&node);
        } else {
            if self.map.len() == self.capacity {
                if let Some(tail_node) = &self.tail.borrow().prev {
                    self.map.remove(&tail_node.borrow().key);
                    self.remove(&tail_node.borrow());
                }
            }

            let new_node = CacheNode::new(key, value);
            self.add(&new_node.borrow());
            self.map.insert(key, new_node);
        }
    }

    fn remove(&self, node: &CacheNode) {
        let prev = node.prev.clone().unwrap();
        let next = node.next.clone().unwrap();
        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev);
    }

    fn add(&self, node: &CacheNode) {
        let next = self.head.borrow().next.clone().unwrap();
        self.head.borrow_mut().next = Some(Rc::new(RefCell::new(node.clone())));
        node.next = Some(next);
        next.borrow_mut().prev = Some(Rc::new(RefCell::new(node.clone())));
        node.prev = Some(self.head.clone());
    }
}
