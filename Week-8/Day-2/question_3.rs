// Question 3: Longest Repeating Character Replacement
// How to find the length of the longest substring where at most k replacements can be made in the string using Rust?

fn question_3(s: String, k: i32) -> i32 {
    let mut count = [0; 26];
    let mut max_len = 0;
    let mut left = 0;
    let mut max_count = 0;

    for right in 0..s.len() {
        let right_char = s.chars().nth(right).unwrap() as usize - 'A' as usize;
        count[right_char] += 1;

        max_count = max_count.max(count[right_char]);

        if right - left + 1 - max_count as i32 > k {
            let left_char = s.chars().nth(left).unwrap() as usize - 'A' as usize;
            count[left_char] -= 1;
            left += 1;
        }

        max_len = max_len.max(right - left + 1);
    }

    max_len
}
