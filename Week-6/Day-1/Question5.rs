// 207. Course Schedule

// There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.

// For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
// Return true if you can finish all courses. Otherwise, return false.

 

// Example 1:

// Input: numCourses = 2, prerequisites = [[1,0]]
// Output: true
// Explanation: There are a total of 2 courses to take. 
// To take course 1 you should have finished course 0. So it is possible.
// Example 2:

// Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
// Output: false
// Explanation: There are a total of 2 courses to take. 
// To take course 1 you should have finished course 0, and to take course 0 you should also have finished course 1. So it is impossible.
 

// Constraints:

// 1 <= numCourses <= 2000
// 0 <= prerequisites.length <= 5000
// prerequisites[i].length == 2
// 0 <= ai, bi < numCourses
// All the pairs prerequisites[i] are unique.

use std::collections::{HashMap, VecDeque};

struct Solution;

impl Solution {
    pub fn can_finish(n: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = n as usize; // Convert n to usize for indexing
        let mut adj: Vec<Vec<i32>> = vec![vec![]; n];
        let mut indegree: Vec<i32> = vec![0; n];
        let mut ans: Vec<i32> = Vec::new();

        // Build the adjacency list and indegree array
        for pair in prerequisites {
            let course = pair[0];
            let prerequisite = pair[1];
            adj[prerequisite as usize].push(course);
            indegree[course as usize] += 1;
        }

        let mut queue: VecDeque<i32> = VecDeque::new();
        for i in 0..n {
            if indegree[i] == 0 {
                queue.push_back(i as i32);
            }
        }

        while let Some(current) = queue.pop_front() {
            ans.push(current);

            for &next_course in &adj[current as usize] {
                indegree[next_course as usize] -= 1;
                if indegree[next_course as usize] == 0 {
                    queue.push_back(next_course);
                }
            }
        }

        ans.len() == n as usize
    }
}