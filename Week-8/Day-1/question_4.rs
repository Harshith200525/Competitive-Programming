// Question 4: Max Points on a Line
// How to find the maximum number of points that lie on the same straight line in a coordinate plane?

use std::collections::HashMap;

fn question_4(points: Vec<Vec<i32>>) -> i32 {
    if points.len() <= 2 {
        return points.len() as i32;
    }

    let mut max_points = 0;
    for i in 0..points.len() {
        let mut slopes = HashMap::new();
        let mut duplicate = 1;
        let mut vertical = 0;
        let mut current_max = 0;

        for j in i + 1..points.len() {
            if points[i][0] == points[j][0] && points[i][1] == points[j][1] {
                duplicate += 1;
            } else if points[i][0] == points[j][0] {
                vertical += 1;
            } else {
                let slope = (points[j][1] - points[i][1]) as f64 / (points[j][0] - points[i][0]) as f64;
                *slopes.entry(slope).or_insert(0) += 1;
                current_max = current_max.max(*slopes.get(&slope).unwrap());
            }
        }

        current_max = current_max.max(vertical);
        max_points = max_points.max(current_max + duplicate);
    }

    max_points
}
