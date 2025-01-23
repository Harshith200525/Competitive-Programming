// Question 1: Find the Closest Palindrome
// How to find the closest palindrome to a given number using arithmetic and string manipulation?

fn question_1(n: String) -> String {
    let num = n.parse::<i64>().unwrap();
    let mut candidates = vec![
        10_i64.pow(n.len() as u32) + 1,
        10_i64.pow(n.len() as u32 - 1) - 1,
    ];

    let prefix = n[0..(n.len() + 1) / 2].parse::<i64>().unwrap();
    for diff in [-1, 0, 1] {
        let mut candidate = (prefix + diff).to_string();
        let suffix: String = candidate.chars().rev().collect();
        if n.len() % 2 == 0 {
            candidate.push_str(&suffix);
        } else {
            candidate.push_str(&suffix[1..]);
        }
        candidates.push(candidate.parse::<i64>().unwrap());
    }

    candidates.retain(|&x| x != num);
    candidates.sort_by_key(|&x| (num - x).abs().min(x));

    candidates[0].to_string()
}
