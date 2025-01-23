// Question 2: Number of Ways to Paint N × 3 Grid
// How to determine the number of ways to paint a grid with specific constraints using modular arithmetic and dynamic programming?

fn question_2(n: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;

    let mut color_2 = 6; // Two-color patterns for first row
    let mut color_3 = 6; // Three-color patterns for first row

    for _ in 1..n {
        let new_color_2 = (3 * color_2 + 2 * color_3) % MOD;
        let new_color_3 = (2 * color_2 + 2 * color_3) % MOD;
        color_2 = new_color_2;
        color_3 = new_color_3;
    }

    (color_2 + color_3) % MOD
}
