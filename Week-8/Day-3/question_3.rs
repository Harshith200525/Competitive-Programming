// Question 3: Flip String to Monotone Increasing
// How to flip a binary string to make it monotone increasing with the minimum number of flips?

fn question_3(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    let mut left_zeros = vec![0; n + 1];
    let mut right_ones = vec![0; n + 1];
    
    for i in 0..n {
        left_zeros[i + 1] = left_zeros[i] + if s[i] == '0' { 1 } else { 0 };
    }
    
    for i in (0..n).rev() {
        right_ones[i] = right_ones[i + 1] + if s[i] == '1' { 1 } else { 0 };
    }
    
    let mut min_flips = i32::MAX;
    for i in 0..=n {
        min_flips = min_flips.min(left_zeros[i] + right_ones[i]);
    }
    
    min_flips
}
