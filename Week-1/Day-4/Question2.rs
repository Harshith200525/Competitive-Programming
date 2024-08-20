// 31. Next Permutation

// A permutation of an array of integers is an arrangement of its members into a sequence or linear order.

// For example, for arr = [1,2,3], the following are all the permutations of arr: [1,2,3], [1,3,2], [2, 1, 3], [2, 3, 1], [3,1,2], [3,2,1].
// The next permutation of an array of integers is the next lexicographically greater permutation of its integer. More formally, if all the permutations of the array are sorted in one container according to their lexicographical order, then the next permutation of that array is the permutation that follows it in the sorted container. If such arrangement is not possible, the array must be rearranged as the lowest possible order (i.e., sorted in ascending order).

// For example, the next permutation of arr = [1,2,3] is [1,3,2].
// Similarly, the next permutation of arr = [2,3,1] is [3,1,2].
// While the next permutation of arr = [3,2,1] is [1,2,3] because [3,2,1] does not have a lexicographical larger rearrangement.
// Given an array of integers nums, find the next permutation of nums.

// The replacement must be in place and use only constant extra memory.

// Input: nums = [1,2,3]
// Output: [1,3,2]

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
       let mut k = nums.len() as isize - 2;

    while k >= 0 && nums[k as usize] >= nums[k as usize + 1] {
        k -= 1;
    }

    if k == -1 {
        nums.reverse();
        return;
    }

    let mut l = nums.len() - 1;
    while nums[l] <= nums[k as usize] {
        l -= 1;
    }

    nums.swap(k as usize, l);

    let (mut left, mut right) = (k as usize + 1, nums.len() - 1);
    while left < right {
        nums.swap(left, right);
        left += 1;
        right -= 1;
    }
}
}