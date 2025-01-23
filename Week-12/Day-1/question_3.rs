// Question 3: Odd Even Jump
// How to determine the number of valid jumps to reach the end of an array using sorted maps and dynamic programming?

use std::collections::BTreeMap;

fn question_3(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut can_jump_odd = vec![false; n];
    let mut can_jump_even = vec![false; n];
    let mut map = BTreeMap::new();

    can_jump_odd[n - 1] = true;
    can_jump_even[n - 1] = true;
    map.insert(arr[n - 1], n - 1);

    for i in (0..n - 1).rev() {
        if let Some((&_, &next)) = map.range(arr[i]..).next() {
            can_jump_odd[i] = can_jump_even[next];
        }

        if let Some((&_, &next)) = map.range(..=arr[i]).next_back() {
            can_jump_even[i] = can_jump_odd[next];
        }

        map.insert(arr[i], i);
    }

    can_jump_odd.into_iter().filter(|&x| x).count() as i32
}
