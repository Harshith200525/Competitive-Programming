// Question 1: Avoid Flood in The City
// How to schedule dry days to avoid floods in a series of lakes, using a strategy with available dry days and binary search?

use std::collections::HashMap;
use std::cmp::min;

fn question_1(rains: Vec<i32>) -> Vec<i32> {
    let mut result = vec![-1; rains.len()];
    let mut lake_last_rain = HashMap::new();
    let mut dry_days = Vec::new();
    
    for (day, lake) in rains.iter().enumerate() {
        if *lake == 0 {
            dry_days.push(day);
        } else {
            if let Some(last_rain_day) = lake_last_rain.get(lake) {
                if let Some(dry_day_index) = dry_days.iter().position(|&x| x > *last_rain_day) {
                    let dry_day = dry_days.remove(dry_day_index);
                    result[dry_day] = *lake;
                } else {
                    return vec![]; // Not possible to avoid the flood
                }
            }
            lake_last_rain.insert(lake, day);
        }
    }

    result
}
