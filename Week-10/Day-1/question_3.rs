// Question 3: Frog Jump
// Can a frog cross a river by making a sequence of valid jumps?

use std::collections::HashSet;

fn question_3(stones: Vec<i32>) -> bool {
    if stones[1] != 1 {
        return false;
    }

    let mut stone_map = HashSet::new();
    let mut positions = HashMap::new();

    for &stone in &stones {
        stone_map.insert(stone);
        positions.insert(stone, HashSet::new());
    }

    positions.get_mut(&stones[0]).unwrap().insert(1);

    for &stone in &stones {
        for &jump in positions.get(&stone).unwrap() {
            for step in [jump - 1, jump, jump + 1] {
                if step > 0 && stone_map.contains(&(stone + step)) {
                    positions.get_mut(&(stone + step)).unwrap().insert(step);
                }
            }
        }
    }

    !positions.get(&stones[stones.len() - 1]).unwrap().is_empty()
}
