// 155. Min Stack

// Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

// Implement the MinStack class:

// MinStack() initializes the stack object.
// void push(int val) pushes the element val onto the stack.
// void pop() removes the element on the top of the stack.
// int top() gets the top element of the stack.
// int getMin() retrieves the minimum element in the stack.
// You must implement a solution with O(1) time complexity for each function.

 

// Example 1:

// Input
// ["MinStack","push","push","push","getMin","pop","top","getMin"]
// [[],[-2],[0],[-3],[],[],[],[]]

// Output
// [null,null,null,null,-3,null,0,-2]

// Explanation
// MinStack minStack = new MinStack();
// minStack.push(-2);
// minStack.push(0);
// minStack.push(-3);
// minStack.getMin(); // return -3
// minStack.pop();
// minStack.top();    // return 0
// minStack.getMin(); // return -2
 

// Constraints:

// -231 <= val <= 231 - 1
// Methods pop, top and getMin operations will always be called on non-empty stacks.
// At most 3 * 104 calls will be made to push, pop, top, and getMin.

use std::collections::VecDeque;

struct MinStack {
    stack: VecDeque<(i32, i32)>, // (value, min_value)
}

impl MinStack {
    // Create a new instance of MinStack
    fn new() -> Self {
        MinStack {
            stack: VecDeque::new(),
        }
    }
    
    // Push a value onto the stack
    fn push(&mut self, val: i32) {
        let min_val = self.get_min();
        let current_min = if min_val.is_none() || min_val.unwrap() > val {
            val
        } else {
            min_val.unwrap()
        };
        self.stack.push_back((val, current_min));
    }
    
    // Pop the top value from the stack
    fn pop(&mut self) {
        self.stack.pop_back();
    }
    
    // Get the top value of the stack
    fn top(&self) -> Option<i32> {
        self.stack.back().map(|&(val, _)| val)
    }
    
    // Get the minimum value from the stack
    fn get_min(&self) -> Option<i32> {
        self.stack.back().map(|&(_, min_val)| min_val)
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let mut obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top().unwrap();
 * let ret_4: i32 = obj.get_min().unwrap();
 */
