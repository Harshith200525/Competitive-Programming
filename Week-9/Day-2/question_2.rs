// Question 2: Parallel Courses
// How to determine the least number of semesters required to complete all courses, considering their prerequisites?

use std::collections::{HashMap, VecDeque};

fn question_2(n: i32, relations: Vec<Vec<i32>>) -> i32 {
    let mut in_degree = vec![0; n as usize];
    let mut graph = vec![vec![]; n as usize];
    
    for relation in relations {
        let (x, y) = (relation[0] - 1, relation[1] - 1);
        in_degree[y as usize] += 1;
        graph[x as usize].push(y);
    }

    let mut queue: VecDeque<i32> = VecDeque::new();
    for i in 0..n {
        if in_degree[i as usize] == 0 {
            queue.push_back(i);
        }
    }

    let mut semester = 0;
    while !queue.is_empty() {
        let size = queue.len();
        for _ in 0..size {
            let course = queue.pop_front().unwrap();
            for &neighbor in &graph[course as usize] {
                in_degree[neighbor as usize] -= 1;
                if in_degree[neighbor as usize] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
        semester += 1;
    }

    if in_degree.iter().any(|&x| x > 0) {
        -1 // If there's a cycle
    } else {
        semester
    }
}
