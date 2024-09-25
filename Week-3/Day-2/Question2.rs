// 57. Insert Interval

// You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi] represent the start and the end of the ith interval and intervals is sorted in ascending order by starti. You are also given an interval newInterval = [start, end] that represents the start and end of another interval.

// Insert newInterval into intervals such that intervals is still sorted in ascending order by starti and intervals still does not have any overlapping intervals (merge overlapping intervals if necessary).

// Return intervals after the insertion.

// Note that you don't need to modify intervals in-place. You can make a new array and return it.

 

// Example 1:

// Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
// Output: [[1,5],[6,9]]
// Example 2:

// Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
// Output: [[1,2],[3,10],[12,16]]
// Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
 

// Constraints:

// 0 <= intervals.length <= 104
// intervals[i].length == 2
// 0 <= starti <= endi <= 105
// intervals is sorted by starti in ascending order.
// newInterval.length == 2
// 0 <= start <= end <= 105

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut output = Vec::new();
        let new_start = new_interval[0];
        let new_end = new_interval[1];

        let mut i = 0;
        let n = intervals.len();

        // Add all intervals that come before the new interval
        while i < n && new_start > intervals[i][0] {
            output.push(intervals[i].clone());
            i += 1;
        }

        // Merge new_interval with the last interval in output if necessary
        if output.is_empty() || output.last().unwrap()[1] < new_start {
            output.push(new_interval);
        } else {
            let last = output.last_mut().unwrap();
            last[1] = last[1].max(new_end);
        }

        // Add remaining intervals, merging if necessary
        while i < n {
            let start = intervals[i][0];
            let end = intervals[i][1];

            if output.last().unwrap()[1] < start {
                output.push(vec![start, end]);
            } else {
                let last = output.last_mut().unwrap();
                last[1] = last[1].max(end);
            }
            i += 1;
        }

        return output;
    }
}