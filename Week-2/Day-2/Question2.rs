// 210. Course Schedule II

// There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.

// For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
// Return the ordering of courses you should take to finish all courses. If there are many valid answers, return any of them. If it is impossible to finish all courses, return an empty array.

 

// Example 1:

// Input: numCourses = 2, prerequisites = [[1,0]]
// Output: [0,1]
// Explanation: There are a total of 2 courses to take. To take course 1 you should have finished course 0. So the correct course order is [0,1].
// Example 2:

// Input: numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
// Output: [0,2,1,3]
// Explanation: There are a total of 4 courses to take. To take course 3 you should have finished both courses 1 and 2. Both courses 1 and 2 should be taken after you finished course 0.
// So one correct course order is [0,1,2,3]. Another correct ordering is [0,2,1,3].
// Example 3:

// Input: numCourses = 1, prerequisites = []
// Output: [0]
 

// Constraints:

// 1 <= numCourses <= 2000
// 0 <= prerequisites.length <= numCourses * (numCourses - 1)
// prerequisites[i].length == 2
// 0 <= ai, bi < numCourses
// ai != bi
// All the pairs [ai, bi] are distinct.

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut sorted_order = Vec::new();

        if n == 0 {
            return sorted_order;
        }

        let mut in_degree = vec![0; n];
        let mut graph = vec![Vec::new(); n];

        for prereq in prerequisites {
            let child = prereq[0] as usize;
            let parent = prereq[1] as usize;
            graph[parent].push(child);
            in_degree[child] += 1;
        }

        let mut sources = VecDeque::new();
        for i in 0..n {
            if in_degree[i] == 0 {
                sources.push_back(i as i32);
            }
        }

        while let Some(vertex) = sources.pop_front() {
            sorted_order.push(vertex);

            for &child in &graph[vertex as usize] {
                in_degree[child] -= 1;
                if in_degree[child] == 0 {
                    sources.push_back(child as i32);
                }
            }
        }

        if sorted_order.len() != n {
            return Vec::new();
        }


        return sorted_order;
    }
}