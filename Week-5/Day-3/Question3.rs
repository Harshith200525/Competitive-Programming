// 56. Merge Intervals

// Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.

 

// Example 1:

// Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
// Output: [[1,6],[8,10],[15,18]]
// Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
// Example 2:

// Input: intervals = [[1,4],[4,5]]
// Output: [[1,5]]
// Explanation: Intervals [1,4] and [4,5] are considered overlapping.
 

// Constraints:

// 1 <= intervals.length <= 104
// intervals[i].length == 2
// 0 <= starti <= endi <= 104

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut merged = Vec::new();
        let mut intervals = intervals.clone();

        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut prev = intervals[0].clone();

        for interval in intervals.iter().skip(1) {
            if interval[0] <= prev[1] {
                prev[1] = std::cmp::max(prev[1], interval[1]);
            } else {
                merged.push(prev);
                prev = interval.clone();
            }
        }

        merged.push(prev);

        return merged;
    }
}