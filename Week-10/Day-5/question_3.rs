// Question 3: Count of Smaller Numbers After Self
// How to count smaller numbers appearing after each element in an array using a Binary Indexed Tree?

fn question_3(nums: Vec<i32>) -> Vec<i32> {
    let offset = 10_000; // Shift values to positive range
    let size = 20_001; // Range of values: -10,000 to 10,000
    let mut bit = vec![0; size];
    let mut result = vec![0; nums.len()];

    fn update(bit: &mut Vec<i32>, index: usize, value: i32) {
        let mut index = index + 1;
        while index < bit.len() {
            bit[index] += value;
            index += index & -index;
        }
    }

    fn query(bit: &Vec<i32>, index: usize) -> i32 {
        let mut sum = 0;
        let mut index = index + 1;
        while index > 0 {
            sum += bit[index];
            index -= index & -index;
        }
        sum
    }

    for i in (0..nums.len()).rev() {
        let rank = (nums[i] + offset) as usize;
        result[i] = query(&bit, rank - 1);
        update(&mut bit, rank, 1);
    }

    result
}
