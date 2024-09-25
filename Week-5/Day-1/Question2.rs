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

use std::collections::{VecDeque, HashMap};

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut in_degree = vec![0; num_courses as usize];
        let mut graph: Vec<Vec<i32>> = vec![vec![]; num_courses as usize];

        // Build the graph and in_degree array
        for edge in prerequisites {
            let parent = edge[1];
            let child = edge[0];
            graph[parent as usize].push(child);
            in_degree[child as usize] += 1;
        }

        let mut sources: VecDeque<i32> = VecDeque::new();
        // Enqueue nodes with no prerequisites
        for i in 0..num_courses {
            if in_degree[i as usize] == 0 {
                sources.push_back(i);
            }
        }

        let mut counter = 0;

        // Process nodes with zero in-degrees
        while let Some(course) = sources.pop_front() {
            counter += 1;
            for &child in &graph[course as usize] {
                in_degree[child as usize] -= 1;
                if in_degree[child as usize] == 0 {
                    sources.push_back(child);
                }
            }
        }

        // If we processed all courses, return true
        counter == num_courses
    }
}
