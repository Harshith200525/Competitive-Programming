// Question 1: Maximum Profit in Job Scheduling
// How to maximize profit by scheduling non-overlapping jobs?

fn question_1(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let mut jobs: Vec<(i32, i32, i32)> = start_time
        .iter()
        .zip(end_time.iter())
        .zip(profit.iter())
        .map(|((&s, &e), &p)| (s, e, p))
        .collect();

    jobs.sort_by_key(|&(_, e, _)| e);

    let mut dp = vec![0];
    let mut end_times = vec![0];

    for (start, end, p) in jobs {
        let i = end_times.binary_search(&(start)).unwrap_or_else(|x| x);
        let max_profit = dp[i] + p;

        if max_profit > *dp.last().unwrap() {
            dp.push(max_profit);
            end_times.push(end);
        }
    }

    *dp.last().unwrap()
}
