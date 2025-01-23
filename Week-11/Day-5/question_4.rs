// Question 4: Maximum Length of a Concatenated String with Unique Characters
// How to find the longest string formed by concatenating unique-character strings using backtracking and bit masking?

fn question_4(arr: Vec<String>) -> i32 {
    fn is_unique(s: &str) -> Option<i32> {
        let mut bitmask = 0;
        for c in s.chars() {
            let bit = 1 << (c as u8 - b'a');
            if bitmask & bit != 0 {
                return None;
            }
            bitmask |= bit;
        }
        Some(bitmask)
    }

    fn backtrack(arr: &[i32], index: usize, mask: i32, max_len: &mut i32) {
        *max_len = (*max_len).max(mask.count_ones() as i32);
        for i in index..arr.len() {
            if mask & arr[i] == 0 {
                backtrack(arr, i + 1, mask | arr[i], max_len);
            }
        }
    }

    let filtered: Vec<i32> = arr
        .into_iter()
        .filter_map(|s| is_unique(&s))
        .collect();

    let mut max_len = 0;
    backtrack(&filtered, 0, 0, &mut max_len);
    max_len
}
