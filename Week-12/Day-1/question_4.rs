// Question 4: Race Car
// How to find the shortest sequence of commands to reach a target position using BFS?

use std::collections::{HashSet, VecDeque};

fn question_4(target: i32) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((0, 1, 0)); // (position, speed, steps)

    while let Some((pos, speed, steps)) = queue.pop_front() {
        if pos == target {
            return steps;
        }

        if !visited.insert((pos, speed)) {
            continue;
        }

        queue.push_back((pos + speed, speed * 2, steps + 1));
        if (pos + speed > target && speed > 0) || (pos + speed < target && speed < 0) {
            queue.push_back((pos, if speed > 0 { -1 } else { 1 }, steps + 1));
        }
    }

    -1
}
