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

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut adj_list = HashMap::new();
        let mut cache = HashMap::new();

        // Step 1: Build adjacency list
        for prereq in prerequisites {
            let course = prereq[0];
            let prerequisite = prereq[1];
            adj_list.entry(prerequisite).or_insert_with(Vec::new).push(course);
        }

        // Step 2: Define helper function for DFS
        fn helper(prereq: i32, adj_list: &HashMap<i32, Vec<i32>>, cache: &mut HashMap<i32, bool>, visited: &mut HashSet<i32>) -> bool {
            if visited.contains(&prereq) {
                return false;
            }
            if let Some(&cached_result) = cache.get(&prereq) {
                return cached_result;
            }
            visited.insert(prereq);
            let mut result = true;
            if let Some(courses) = adj_list.get(&prereq) {
                for &course in courses {
                    if !helper(course, adj_list, cache, visited) {
                        result = false;
                        break;
                    }
                }
            }
            visited.remove(&prereq);
            cache.insert(prereq, result);
            result
        }

        // Step 3: Iterate through each course and check for cycles
        let mut visited = HashSet::new();
        for &prereq in adj_list.keys() {
            if !helper(prereq, &adj_list, &mut cache, &mut visited) {
                return false;
            }
        }

        true
    }
}
