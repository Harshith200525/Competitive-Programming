// Problem Statement: Alice and Bob's Blobby Volley Game
// Alice and Bob are playing a game of Blobby Volley. In this game, on each turn, one player is the server and the other player is the receiver. Initially, Alice is the server, and Bob is the receiver.

// The rules are as follows:

// If the server wins the point in a turn, their score increases by 1, and they remain the server for the next turn.
// If the receiver wins the point in a turn, their score does not increase. However, they become the server in the next turn.
// Both players start with a score of 0, and they play N turns. The winner of each turn is given to you as a string consisting of characters 'A' and 'B':

// 'A' denotes that Alice won the point.
// 'B' denotes that Bob won the point.
// Your task is to find and print the scores of both Alice and Bob after the N turns.

// Input
// 4
// 3
// AAA
// 4
// BBBB
// 5
// ABABB
// 5
// BABAB

// Output
// 3 0
// 0 3
// 1 1
// 0 0

use std::io::{self, BufRead};

fn solve_test_case(server: &char, n: usize, results: &str) {
    // Your logic here.
    // You have the initial server ('A' or 'B'), the number of turns (n), and the results string.
    let mut alice = 0;
    let mut bob = 0;
    
    let some: Vec<char> = results.chars().collect();
    
    if some[0] == 'A'{
        alice += 1;
    }
    
    for i in 0..n-1{
        if some[i] == some[i+1]{
            if some[i] == 'A'{
                alice += 1;
            }else if some[i] == 'B'{
                bob += 1;
            }
        }
    }
    
    println!("{} {}", alice, bob);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // First line is the number of test cases
    let t: usize = lines.next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Expected a number");

    for _ in 0..t {
        // Read the number of turns (N)
        let n: usize = lines.next()
            .unwrap()
            .unwrap()
            .trim()
            .parse()
            .expect("Expected a number");

        // Read the results string (S)
        let results = lines.next()
            .unwrap()
            .unwrap()
            .trim()
            .to_string();
        
        // Initially, Alice is the server
        let initial_server = 'A';

        solve_test_case(&initial_server, n, &results);
    }
}