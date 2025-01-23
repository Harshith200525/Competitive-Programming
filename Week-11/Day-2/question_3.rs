// Question 3: Course Schedule III
// How to schedule courses to maximize the number taken within their deadlines?

use std::collections::BinaryHeap;

fn question_3(courses: Vec<Vec<i32>>) -> i32 {
    let mut courses = courses;
    courses.sort_by_key(|course| course[1]);

    let mut total_time = 0;
    let mut max_heap = BinaryHeap::new();

    for course in courses {
        let duration = course[0];
        let deadline = course[1];

        total_time += duration;
        max_heap.push(duration);

        if total_time > deadline {
            total_time -= max_heap.pop().unwrap();
        }
    }

    max_heap.len() as i32
}
