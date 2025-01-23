// Question 2: Median of Two Sorted Arrays
// How to find the median of two sorted arrays in logarithmic time?

fn question_2(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (mut a, mut b) = if nums1.len() > nums2.len() {
        (nums2, nums1)
    } else {
        (nums1, nums2)
    };

    let (mut left, mut right) = (0, a.len());
    let total = a.len() + b.len();
    let half = total / 2;

    while left <= right {
        let i = (left + right) / 2;
        let j = half - i;

        let a_left = if i > 0 { a[i - 1] } else { i32::MIN };
        let a_right = if i < a.len() { a[i] } else { i32::MAX };

        let b_left = if j > 0 { b[j - 1] } else { i32::MIN };
        let b_right = if j < b.len() { b[j] } else { i32::MAX };

        if a_left <= b_right && b_left <= a_right {
            if total % 2 == 0 {
                return (a_left.max(b_left) + a_right.min(b_right)) as f64 / 2.0;
            } else {
                return a_left.max(b_left) as f64;
            }
        } else if a_left > b_right {
            right = i - 1;
        } else {
            left = i + 1;
        }
    }

    unreachable!()
}
