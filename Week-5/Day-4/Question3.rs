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

struct Node {
    key: i32,
    val: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            key,
            val,
            prev: None,
            next: None,
        }))
    }
}

struct LRUCache {
    cap: usize,
    map: HashMap<i32, Rc<RefCell<Node>>>,
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let head = Node::new(-1, -1);
        let tail = Node::new(-1, -1);
        {
            let mut head = head.borrow_mut();
            let mut tail = tail.borrow_mut();
            head.next = Some(tail.clone());
            tail.prev = Some(head.clone());
        }
        LRUCache {
            cap: capacity as usize,
            map: HashMap::new(),
            head,
            tail,
        }
    }

    fn add_node(&self, node: Rc<RefCell<Node>>) {
        let mut node = node.borrow_mut();
        let mut head = self.head.borrow_mut();
        let mut next = head.next.as_ref().unwrap().borrow_mut();
        node.prev = Some(head.clone());
        node.next = Some(next.clone());
        head.next = Some(node.clone());
        next.prev = Some(node.clone());
    }

    fn remove_node(&self, node: Rc<RefCell<Node>>) {
        let mut node = node.borrow_mut();
        let prev = node.prev.clone().unwrap();
        let next = node.next.clone().unwrap();
        let mut prev = prev.borrow_mut();
        let mut next = next.borrow_mut();
        prev.next = Some(next.clone());
        next.prev = Some(prev.clone());
    }

    fn get(&self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key) {
            let node = node.clone();
            let value = node.borrow().val;
            self.remove_node(node.clone());
            self.add_node(node.clone());
            value
        } else {
            -1
        }
    }

    fn put(&self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key) {
            let node = node.clone();
            self.remove_node(node.clone());
            node.borrow_mut().val = value;
            self.add_node(node.clone());
        } else {
            if self.map.len() == self.cap {
                if let Some(tail_prev) = self.tail.borrow().prev.clone() {
                    let key_to_remove = tail_prev.borrow().key;
                    self.remove_node(tail_prev.clone());
                    self.map.remove(&key_to_remove);
                }
            }
            let new_node = Node::new(key, value);
            self.add_node(new_node.clone());
            self.map.insert(key, new_node);
        }
    }
}

