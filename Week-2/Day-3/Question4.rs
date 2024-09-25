// 77. Combinations

// Given two integers n and k, return all possible combinations of k numbers chosen from the range [1, n].

// You may return the answer in any order.

 

// Example 1:

// Input: n = 4, k = 2
// Output: [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
// Explanation: There are 4 choose 2 = 6 total combinations.
// Note that combinations are unordered, i.e., [1,2] and [2,1] are considered to be the same combination.
// Example 2:

// Input: n = 1, k = 1
// Output: [[1]]
// Explanation: There is 1 choose 1 = 1 total combination.
 

// Constraints:

// 1 <= n <= 20
// 1 <= k <= n

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut comb = Vec::new();

        fn backtrack(start: i32, n: i32, k: i32, comb: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if comb.len() == k as usize {
                res.push(comb.clone());
                return;
            }

            for num in start..=n {
            comb.push(num);
            backtrack(num + 1, n, k, comb, res);
            comb.pop();
            }
        }

        backtrack(1, n, k, &mut comb, &mut res);
        return res;
    }
}