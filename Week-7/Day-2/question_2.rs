// Question 2: Largest Rectangle in Histogram
// How to find the largest rectangle area in a histogram using Rust?

fn question_2(heights: Vec<i32>) -> i32 {
    let mut stack = Vec::new();
    let mut max_area = 0;
    let mut heights = heights;
    heights.push(0);

    for i in 0..heights.len() {
        while let Some(&top) = stack.last() {
            if heights[i] >= heights[top] {
                break;
            }
            stack.pop();
            let height = heights[top];
            let width = if let Some(&last) = stack.last() {
                i - last - 1
            } else {
                i
            };
            max_area = max_area.max(height * width as i32);
        }
        stack.push(i);
    }

    max_area
}
