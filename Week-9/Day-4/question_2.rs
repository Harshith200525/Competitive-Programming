// Question 2: Count Subarrays with Median K
// How to find the number of subarrays with a median equal to K using prefix sums and hash maps?

use std::collections::HashMap;

fn question_2(nums: Vec<i32>, k: i32) -> i32 {
    let mut prefix = 0;
    let mut count = 0;
    let mut map = HashMap::new();
    map.insert(0, 1);

    for &num in &nums {
        if num < k {
            prefix -= 1;
        } else if num > k {
            prefix += 1;
        }

        if map.contains_key(&(prefix)) {
            count += map[&prefix];
        }

        *map.entry(prefix).or_insert(0) += 1;
    }

    count
}
