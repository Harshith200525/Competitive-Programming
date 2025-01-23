// Question 4: Restore the Array From Adjacent Pairs
// How to restore an array from two arrays that contain adjacent pairs of elements?

use std::collections::HashMap;

fn question_4(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph = HashMap::new();
    let mut degree = HashMap::new();

    for pair in adjacent_pairs {
        let (a, b) = (pair[0], pair[1]);
        graph.entry(a).or_insert(vec![]).push(b);
        graph.entry(b).or_insert(vec![]).push(a);
        *degree.entry(a).or_insert(0) += 1;
        *degree.entry(b).or_insert(0) += 1;
    }

    let start = *degree.iter().find(|&(_, &v)| v == 1).unwrap().0;
    let mut result = vec![start];
    let mut current = start;

    while let Some(&next) = graph.get(&current).and_then(|v| v.first()) {
        result.push(next);
        current = next;
    }

    result
}
