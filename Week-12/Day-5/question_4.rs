// Question 4: Minimize Maximum Distance to Gas Station
// How to position additional gas stations to minimize the maximum distance between stations using binary search?

fn question_4(stations: Vec<i32>, k: i32) -> f64 {
    let mut left = 0.0;
    let mut right = *stations.last().unwrap() as f64 - stations[0] as f64;

    let is_possible = |d: f64| -> bool {
        let mut count = 0;
        for i in 1..stations.len() {
            count += ((stations[i] - stations[i - 1]) as f64 / d).ceil() as i32 - 1;
        }
        count <= k
    };

    while right - left > 1e-6 {
        let mid = left + (right - left) / 2.0;
        if is_possible(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }

    right
}
