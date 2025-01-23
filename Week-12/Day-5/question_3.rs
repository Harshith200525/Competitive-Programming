// Question 3: Profit Maximization in Inventory
// How to sell inventory items to maximize profits using binary heaps?

use std::collections::BinaryHeap;

fn question_3(inventory: Vec<i32>, orders: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;

    let mut heap = BinaryHeap::new();
    for &item in &inventory {
        heap.push(item as i64);
    }

    let mut orders = orders as i64;
    let mut profit = 0;

    while orders > 0 {
        let current = heap.pop().unwrap();
        let next = *heap.peek().unwrap_or(&0);
        let count = current - next;
        let take = count.min(orders);

        let total = take * (current + current - take + 1) / 2;
        profit = (profit + total) % MOD;

        if count > orders {
            heap.push(current - orders);
        }

        orders -= take;
    }

    profit as i32
}
