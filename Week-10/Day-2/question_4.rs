// Question 4: Split Array Largest Sum
// How to split an array into subarrays to minimize the largest sum?

fn question_4(nums: Vec<i32>, m: i32) -> i32 {
    let mut left = *nums.iter().max().unwrap();
    let mut right: i32 = nums.iter().sum();

    fn can_split(nums: &Vec<i32>, m: i32, max_sum: i32) -> bool {
        let mut current_sum = 0;
        let mut count = 1;

        for &num in nums {
            if current_sum + num > max_sum {
                count += 1;
                current_sum = num;

                if count > m {
                    return false;
                }
            } else {
                current_sum += num;
            }
        }

        true
    }

    while left < right {
        let mid = left + (right - left) / 2;

        if can_split(&nums, m, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}
