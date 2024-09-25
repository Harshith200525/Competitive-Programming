// 78. Subsets

// Given an integer array nums of unique elements, return all possible 
// subsets
//  (the power set).

// The solution set must not contain duplicate subsets. Return the solution in any order.

 

// Example 1:

// Input: nums = [1,2,3]
// Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
// Example 2:

// Input: nums = [0]
// Output: [[],[0]]
 

// Constraints:

// 1 <= nums.length <= 10
// -10 <= nums[i] <= 10
// All the numbers of nums are unique.

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut subset = Vec::new();

        fn create_subset(i: usize, nums: &Vec<i32>, subset: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if i == nums.len() {
                res.push(subset.clone());
                return;
            }

            // Include the current element in the subset
            subset.push(nums[i]);
            create_subset(i + 1, nums, subset, res);

            // Exclude the current element from the subset
            subset.pop();
            create_subset(i + 1, nums, subset, res);
        }

        create_subset(0, &nums, &mut subset, &mut res);
        res
    }
}
