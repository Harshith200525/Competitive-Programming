// Question 2: Russian Doll Envelopes
// How to find the maximum number of envelopes that can be nested based on their dimensions using dynamic programming?

fn question_2(envelopes: Vec<Vec<i32>>) -> i32 {
    let mut envelopes = envelopes;
    envelopes.sort_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));

    let n = envelopes.len();
    let mut dp = vec![1; n];

    for i in 1..n {
        for j in 0..i {
            if envelopes[i][1] > envelopes[j][1] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }

    dp.into_iter().max().unwrap_or(0)
}
