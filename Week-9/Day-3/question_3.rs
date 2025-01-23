// Question 3: Design Front Middle Back Queue
// How to design a data structure that supports insertion and deletion at the front, middle, and back?

use std::collections::VecDeque;

pub struct FrontMiddleBackQueue {
    front: VecDeque<i32>,
    back: VecDeque<i32>,
}

impl FrontMiddleBackQueue {
    pub fn new() -> Self {
        FrontMiddleBackQueue {
            front: VecDeque::new(),
            back: VecDeque::new(),
        }
    }

    pub fn push_front(&mut self, val: i32) {
        self.front.push_front(val);
        self.balance();
    }

    pub fn push_middle(&mut self, val: i32) {
        if self.front.len() > self.back.len() {
            self.back.push_front(self.front.pop_back().unwrap());
        }
        self.front.push_back(val);
        self.balance();
    }

    pub fn push_back(&mut self, val: i32) {
        self.back.push_back(val);
        self.balance();
    }

    pub fn pop_front(&mut self) -> i32 {
        if !self.front.is_empty() {
            self.front.pop_front().unwrap()
        } else {
            self.back.pop_front().unwrap()
        }
    }

    pub fn pop_middle(&mut self) -> i32 {
        if self.front.len() > self.back.len() {
            self.front.pop_back().unwrap()
        } else {
            self.back.pop_front().unwrap()
        }
    }

    pub fn pop_back(&mut self) -> i32 {
        if !self.back.is_empty() {
            self.back.pop_back().unwrap()
        } else {
            self.front.pop_back().unwrap()
        }
    }

    fn balance(&mut self) {
        if self.front.len() > self.back.len() + 1 {
            self.back.push_front(self.front.pop_back().unwrap());
        } else if self.back.len() > self.front.len() {
            self.front.push_back(self.back.pop_front().unwrap());
        }
    }
}
