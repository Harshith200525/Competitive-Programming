// 84. Largest Rectangle in Histogram

// Given an array of integers heights representing the histogram's bar height where the width of each bar is 1, return the area of the largest rectangle in the histogram.

 

// Example 1:


// Input: heights = [2,1,5,6,2,3]
// Output: 10
// Explanation: The above is a histogram where width of each bar is 1.
// The largest rectangle is shown in the red area, which has an area = 10 units.
// Example 2:


// Input: heights = [2,4]
// Output: 4
 

// Constraints:

// 1 <= heights.length <= 105
// 0 <= heights[i] <= 104

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = vec![-1];
        let mut max_area = 0;
        let n = heights.len();

        for i in 0..n {
            while stack.last() != Some(&-1) && heights[i] <= heights[stack.last().unwrap() as usize] {
                let height = heights[stack.pop().unwrap() as usize];
                let width = i as i32 - stack.last().unwrap() - 1;
                max_area = max_area.max(height * width);
            }
            stack.push(i as i32);
        }

        while stack.last() != Some(&-1) {
            let height = heights[stack.pop().unwrap() as usize];
            let width = n as i32 - stack.last().unwrap() - 1;
            max_area = max_area.max(height * width);
        }

        max_area
    }
}
