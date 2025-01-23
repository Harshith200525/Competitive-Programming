// Question 4: Smallest Rectangle Enclosing Black Pixels
// How to find the smallest rectangle enclosing all black pixels in a binary image?

fn question_4(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 {
    let rows = image.len();
    let cols = image[0].len();

    fn binary_search<F: Fn(usize) -> bool>(low: usize, high: usize, condition: F) -> usize {
        let mut low = low;
        let mut high = high;

        while low < high {
            let mid = low + (high - low) / 2;
            if condition(mid) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        low
    }

    let top = binary_search(0, x as usize, |mid| image[mid].contains(&'1'));
    let bottom = binary_search(x as usize + 1, rows, |mid| !image[mid].contains(&'1'));
    let left = binary_search(0, y as usize, |mid| image.iter().any(|row| row[mid] == '1'));
    let right = binary_search(y as usize + 1, cols, |mid| !image.iter().any(|row| row[mid] == '1'));

    ((bottom - top) * (right - left)) as i32
}
