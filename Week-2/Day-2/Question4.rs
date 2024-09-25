// 216. Combination Sum III

// Find all valid combinations of k numbers that sum up to n such that the following conditions are true:

// Only numbers 1 through 9 are used.
// Each number is used at most once.
// Return a list of all possible valid combinations. The list must not contain the same combination twice, and the combinations may be returned in any order.

 

// Example 1:

// Input: k = 3, n = 7
// Output: [[1,2,4]]
// Explanation:
// 1 + 2 + 4 = 7
// There are no other valid combinations.
// Example 2:

// Input: k = 3, n = 9
// Output: [[1,2,6],[1,3,5],[2,3,4]]
// Explanation:
// 1 + 2 + 6 = 9
// 1 + 3 + 5 = 9
// 2 + 3 + 4 = 9
// There are no other valid combinations.
// Example 3:

// Input: k = 4, n = 1
// Output: []
// Explanation: There are no valid combinations.
// Using 4 different numbers in the range [1,9], the smallest sum we can get is 1+2+3+4 = 10 and since 10 > 1, there are no valid combination.
 

// Constraints:

// 2 <= k <= 9
// 1 <= n <= 60

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
    let mut path = Vec::new();

    fn dfs(index: i32, k: i32, n: i32, path: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>, total: i32) {
        if path.len() == k as usize {
            if total == n {
                ret.push(path.clone());
            }
            return;
        }

        for i in index..=9 {
            let curr = total + i;
            if curr <= n {
                path.push(i);
                dfs(i + 1, k, n, path, ret, curr);
                path.pop();
            }
        }
    }

    dfs(1, k, n, &mut path, &mut ret, 0);
    return ret;
    }
}