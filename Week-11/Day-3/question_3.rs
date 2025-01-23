// Question 3: K Empty Slots
// How to determine the earliest day two flowers bloomed with k empty slots between them using a sliding window and binary search?

fn question_3(flowerbed: Vec<i32>, k: i32) -> i32 {
    let n = flowerbed.len();
    let mut bloom_day = vec![0; n];
    let mut slots = Vec::new();

    for (i, &flower) in flowerbed.iter().enumerate() {
        if flower == 1 {
            bloom_day[i] = 1;
            slots.push(i);
        }
    }

    if slots.len() < 2 {
        return -1;
    }

    let mut res = i32::MAX;
    for i in 0..slots.len() - 1 {
        for j in i + 1..slots.len() {
            let gap = slots[j] - slots[i] - 1;
            if gap == k {
                res = res.min(j as i32 - i as i32);
            }
        }
    }

    if res == i32::MAX {
        return -1;
    } else {
        return res;
    }
}
