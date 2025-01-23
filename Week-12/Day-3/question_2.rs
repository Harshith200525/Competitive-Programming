// Question 2: Remove Boxes
// How to maximize points by removing boxes in an optimal sequence using dynamic programming?

fn question_2(boxes: Vec<i32>) -> i32 {
    let n = boxes.len();
    let mut dp = vec![vec![vec![0; n]; n]; n];

    fn dfs(boxes: &Vec<i32>, l: usize, r: usize, k: usize, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        if l > r {
            return 0;
        }
        if dp[l][r][k] != 0 {
            return dp[l][r][k];
        }

        let mut l = l;
        let mut k = k;

        while l + 1 <= r && boxes[l] == boxes[l + 1] {
            l += 1;
            k += 1;
        }

        dp[l][r][k] = (k + 1) * (k + 1) as i32 + dfs(boxes, l + 1, r, 0, dp);

        for m in l + 1..=r {
            if boxes[m] == boxes[l] {
                dp[l][r][k] = dp[l][r][k].max(
                    dfs(boxes, l + 1, m - 1, 0, dp) + dfs(boxes, m, r, k + 1, dp),
                );
            }
        }

        dp[l][r][k]
    }

    dfs(&boxes, 0, n - 1, 0, &mut dp)
}
