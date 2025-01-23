// Question 1: Rectangle Area II
// How to calculate the total area covered by multiple rectangles using a sweep line algorithm?

fn question_1(rectangles: Vec<Vec<i32>>) -> i32 {
    let mut events = Vec::new();
    for rect in &rectangles {
        events.push((rect[1], 1, rect[0], rect[2])); // Start of a rectangle
        events.push((rect[3], -1, rect[0], rect[2])); // End of a rectangle
    }
    events.sort();

    let mut x_intervals = Vec::new();
    let mut last_y = 0;
    let mut area = 0;

    for &(y, typ, x1, x2) in &events {
        let mut active_length = 0;
        let mut prev_x = 0;
        for &(sx, ex) in &x_intervals {
            active_length += (ex - sx).max(0);
            prev_x = ex;
        }

        area += active_length * (y - last_y);
        last_y = y;

        if typ == 1 {
            x_intervals.push((x1, x2));
        } else {
            x_intervals.retain(|&(sx, ex)| sx != x1 || ex != x2);
        }
    }

    area % 1_000_000_007
}
