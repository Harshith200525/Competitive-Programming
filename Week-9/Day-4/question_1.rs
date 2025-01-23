// Question 1: Paint House III
// How to minimize painting costs for a series of houses with neighborhood constraints, using dynamic programming?

fn question_1(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
    let mut dp = vec![vec![vec![-1; target as usize + 1]; n as usize]; m as usize];

    fn paint(i: usize, color: usize, remaining_target: i32, houses: &Vec<i32>, cost: &Vec<Vec<i32>>, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        if remaining_target < 0 {
            return i32::MAX;
        }

        if i == houses.len() {
            return if remaining_target == 0 { 0 } else { i32::MAX };
        }

        if dp[i][color][remaining_target as usize] != -1 {
            return dp[i][color][remaining_target as usize];
        }

        let mut ans = i32::MAX;

        if houses[i] == 0 {
            for c in 0..n as usize {
                if c == color {
                    ans = ans.min(paint(i + 1, c, remaining_target, houses, cost, dp) + cost[i][c]);
                } else {
                    ans = ans.min(paint(i + 1, c, remaining_target - 1, houses, cost, dp) + cost[i][c]);
                }
            }
        } else {
            ans = ans.min(paint(i + 1, houses[i] as usize - 1, remaining_target, houses, cost, dp));
        }

        dp[i][color][remaining_target as usize] = ans;
        ans
    }

    let result = paint(0, 0, target, &houses, &cost, &mut dp);
    if result == i32::MAX {
        -1
    } else {
        result
    }
}
