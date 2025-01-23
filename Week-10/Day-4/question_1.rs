// Question 1: Maximum Points You Can Obtain from Cards
// How to maximize points by selecting cards from either end of an array?

fn question_1(card_points: Vec<i32>, k: usize) -> i32 {
    let total: i32 = card_points.iter().sum();
    if k == card_points.len() {
        return total;
    }

    let n = card_points.len();
    let window_size = n - k;
    let mut current_sum: i32 = card_points[0..window_size].iter().sum();
    let mut min_sum = current_sum;

    for i in window_size..n {
        current_sum += card_points[i] - card_points[i - window_size];
        min_sum = min_sum.min(current_sum);
    }

    total - min_sum
}
