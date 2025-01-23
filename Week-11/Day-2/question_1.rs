// Question 1: Minimum Window Subsequence
// How to find the smallest window in a string that contains a given subsequence?

fn question_1(s1: String, s2: String) -> String {
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();
    let mut start = 0;
    let mut min_len = usize::MAX;

    for i in 0..s1.len() {
        if s1[i] == s2[0] {
            let mut j = 0;
            let mut k = i;

            while k < s1.len() && j < s2.len() {
                if s1[k] == s2[j] {
                    j += 1;
                }
                k += 1;
            }

            if j == s2.len() {
                let mut end = k - 1;
                j -= 1;

                while j >= 0 {
                    if s1[end] == s2[j] {
                        j -= 1;
                    }
                    end -= 1;
                }

                end += 1;
                if k - end < min_len {
                    min_len = k - end;
                    start = end;
                }
            }
        }
    }

    if min_len == usize::MAX {
        "".to_string()
    } else {
        s1[start..start + min_len].iter().collect()
    }
}
