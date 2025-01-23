// Question 3: Bus Routes
// How to find the minimum number of buses needed to travel between two locations, given bus routes?

use std::collections::{HashMap, HashSet, VecDeque};

fn question_3(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    if source == target {
        return 0;
    }

    let mut graph = HashMap::new();
    let mut visited_routes = HashSet::new();
    let mut visited_stops = HashSet::new();
    let mut queue = VecDeque::new();

    // Build graph of bus stops to routes
    for (i, route) in routes.iter().enumerate() {
        for &stop in route {
            graph.entry(stop).or_insert(Vec::new()).push(i);
        }
    }

    // BFS traversal
    queue.push_back((source, 0)); // (current stop, number of buses)
    visited_stops.insert(source);

    while let Some((stop, buses)) = queue.pop_front() {
        if stop == target {
            return buses;
        }

        for &route_idx in graph.get(&stop).unwrap_or(&vec![]) {
            if visited_routes.insert(route_idx) {
                for &next_stop in &routes[route_idx] {
                    if visited_stops.insert(next_stop) {
                        queue.push_back((next_stop, buses + 1));
                    }
                }
            }
        }
    }

    -1
}
