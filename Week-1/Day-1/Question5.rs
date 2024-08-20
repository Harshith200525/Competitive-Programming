// The World Chess Championship 2022 is about to start. A total of 14 Classical games will be played between Chef and Carlsen in the championship. Each game can have one of three outcomes — it can be won by Carlsen, won by Chef, or it can be a draw. The winner of a game receives 2 points, and the loser receives 0 points. If the game is a draw, both players get 1 point each.

// The total prize pool of the championship is 100 * X. At the end of the 14 Classical games, if one player has strictly more points than the other, he is declared the champion and receives 60 * X as his prize money, while the loser receives 40 * X.

// If the total points are tied, then the defending champion Carlsen is declared the winner. In this case, Carlsen receives 55 * X as his prize money, and the other player receives 45 * X.

//Input
// 4
// 100
// CCCCCCCCCCCCCC
// 400
// CDCDCDCDCDCDCD
// 30
// DDCCNNDDDCCNND
// 1
// NNDNNDDDNNDNDN

//Output
// 6000
// 24000
// 1650
// 40

use std::io::{self, BufRead};

fn solve(x: i32, results: &str) {
    // Your logic will go here
    let mut C: usize = 0;
    let mut N: usize = 0;
    
    for i in results.chars(){
        if i == 'C'{
            C += 1;
        }else if i == 'N'{
            N += 1;
        }else{
            C += 1;
            N += 1;
        }
    }
    if C > N{
        println!("{}",60*x);
    }else if N > C{
        println!("{}",40*x);
    }else{
        println!("{}",55*x);
    }
    
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let x: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let results = lines.next().unwrap().unwrap().trim().to_string();

        solve(x, &results);
    }
}