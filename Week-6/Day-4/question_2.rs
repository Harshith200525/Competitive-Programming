// Question 2: Merge Intervals
// How to merge overlapping intervals in a list using Rust?

fn question_2(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() {
        return vec![];
    }

    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut merged = vec![intervals[0].clone()];

    for interval in intervals.iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if interval[0] <= last[1] {
            last[1] = last[1].max(interval[1]);
        } else {
            merged.push(interval.clone());
        }
    }

    merged
}
