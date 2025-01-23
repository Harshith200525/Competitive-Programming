// Question 3: Best Time to Buy and Sell Stock with Cooldown
// How to find the optimal way to buy and sell stocks with a cooldown period using dynamic programming in Rust?

fn question_3(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }
    let (mut sell, mut hold, mut rest) = (0, i32::MIN, 0);
    for price in prices {
        let prev_sell = sell;
        sell = hold + price;
        hold = hold.max(rest - price);
        rest = rest.max(prev_sell);
    }
    sell.max(rest)
}
