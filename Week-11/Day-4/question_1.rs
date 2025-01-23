// Question 1: Network Delay Time
// How to calculate the time it takes for a signal to travel through a network using Dijkstra's algorithm and priority queues?

use std::collections::{BinaryHeap, HashMap};

fn question_1(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let n = n as usize;
    let mut graph = vec![vec![]; n];
    for time in times {
        graph[time[0] as usize - 1].push((time[1] as usize - 1, time[2]));
    }

    let mut dist = vec![i32::MAX; n];
    dist[k as usize - 1] = 0;

    let mut heap = BinaryHeap::new();
    heap.push((0, k as usize - 1));

    while let Some((d, u)) = heap.pop() {
        if d > dist[u] {
            continue;
        }

        for &(v, w) in &graph[u] {
            let new_dist = d + w;
            if new_dist < dist[v] {
                dist[v] = new_dist;
                heap.push((new_dist, v));
            }
        }
    }

    let result = *dist.iter().max().unwrap();
    if result == i32::MAX {
        -1
    } else {
        result
    }
}
