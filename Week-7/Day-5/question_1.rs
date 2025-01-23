// Question 1: Maximal Rectangle
// How to find the largest rectangle containing only 1's in a 2D binary matrix using Rust?

fn question_1(matrix: Vec<Vec<char>>) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    let mut heights = vec![0; matrix[0].len()];
    let mut max_area = 0;

    for row in matrix {
        for (i, &cell) in row.iter().enumerate() {
            heights[i] = if cell == '1' { heights[i] + 1 } else { 0 };
        }

        max_area = max_area.max(largest_rectangle_in_histogram(&heights));
    }

    max_area
}

fn largest_rectangle_in_histogram(heights: &[i32]) -> i32 {
    let mut stack = Vec::new();
    let mut max_area = 0;
    let mut heights = heights.to_vec();
    heights.push(0); // Add a zero at the end to flush remaining bars

    for i in 0..heights.len() {
        while let Some(&top) = stack.last() {
            if heights[i] >= heights[top as usize] {
                break;
            }
            stack.pop();
            let height = heights[top as usize];
            let width = if let Some(&last) = stack.last() {
                (i - last - 1) as i32
            } else {
                i as i32
            };
            max_area = max_area.max(height * width);
        }
        stack.push(i as i32);
    }

    max_area
}
