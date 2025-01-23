// Question 1: Course Schedule II
// How to find the correct order of courses to take based on course prerequisites using Rust?

use std::collections::{HashSet, VecDeque};

fn question_1(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph = vec![vec![]; num_courses as usize];
    let mut in_degree = vec![0; num_courses as usize];
    let mut result = Vec::new();

    // Build graph and in-degree array
    for pre in prerequisites {
        graph[pre[1] as usize].push(pre[0]);
        in_degree[pre[0] as usize] += 1;
    }

    let mut queue = VecDeque::new();
    for i in 0..num_courses {
        if in_degree[i as usize] == 0 {
            queue.push_back(i);
        }
    }

    // Perform topological sort using BFS
    while let Some(course) = queue.pop_front() {
        result.push(course);
        for &next in &graph[course as usize] {
            in_degree[next as usize] -= 1;
            if in_degree[next as usize] == 0 {
                queue.push_back(next);
            }
        }
    }

    if result.len() == num_courses as usize {
        result
    } else {
        Vec::new() // No valid ordering exists (cycle detected)
    }
}
