// Question 2: Max Chunks to Make Sorted II
// How to sort an array in the maximum number of chunks that can be individually sorted to produce the sorted array?

fn question_2(arr: Vec<i32>) -> i32 {
    let mut sorted = arr.clone();
    sorted.sort();
    let mut max_chunk_count = 0;
    let mut left_max = vec![0; arr.len()];
    let mut right_min = vec![0; arr.len()];

    left_max[0] = arr[0];
    for i in 1..arr.len() {
        left_max[i] = left_max[i - 1].max(arr[i]);
    }

    right_min[arr.len() - 1] = sorted[arr.len() - 1];
    for i in (0..arr.len() - 1).rev() {
        right_min[i] = right_min[i + 1].min(sorted[i]);
    }

    for i in 0..arr.len() - 1 {
        if left_max[i] <= right_min[i + 1] {
            max_chunk_count += 1;
        }
    }

    max_chunk_count + 1
}
