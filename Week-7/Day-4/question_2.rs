// Question 2: Palindrome Partitioning
// How to find all possible palindrome partitions of a string using Rust?

fn question_2(s: String) -> Vec<Vec<String>> {
    fn is_palindrome(s: &str) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        for i in 0..n / 2 {
            if s[i] != s[n - i - 1] {
                return false;
            }
        }
        true
    }

    fn backtrack(start: usize, path: &mut Vec<String>, s: &str, res: &mut Vec<Vec<String>>) {
        if start == s.len() {
            res.push(path.clone());
            return;
        }

        for i in start + 1..=s.len() {
            let substring = &s[start..i];
            if is_palindrome(substring) {
                path.push(substring.to_string());
                backtrack(i, path, s, res);
                path.pop();
            }
        }
    }

    let mut res = Vec::new();
    let mut path = Vec::new();
    backtrack(0, &mut path, &s, &mut res);
    res
}
