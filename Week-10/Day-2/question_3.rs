// Question 3: Word Search II
// How to find all words from a dictionary within a grid using a Trie and backtracking?

use std::collections::HashSet;

#[derive(Default)]
struct TrieNode {
    children: std::collections::HashMap<char, TrieNode>,
    word: Option<String>,
}

impl TrieNode {
    fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        node.word = Some(word.to_string());
    }
}

fn question_3(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let mut root = TrieNode::default();
    for word in words {
        root.insert(&word);
    }

    let mut result = HashSet::new();
    let rows = board.len();
    let cols = board[0].len();
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut visited = vec![vec![false; cols]; rows];

    fn backtrack(
        board: &Vec<Vec<char>>,
        i: usize,
        j: usize,
        node: &mut TrieNode,
        visited: &mut Vec<Vec<bool>>,
        result: &mut HashSet<String>,
        directions: &[(i32, i32)],
    ) {
        if let Some(word) = &node.word {
            result.insert(word.clone());
        }

        visited[i][j] = true;

        for &(dx, dy) in directions {
            let ni = i as i32 + dx;
            let nj = j as i32 + dy;

            if ni >= 0 && ni < board.len() as i32 && nj >= 0 && nj < board[0].len() as i32 {
                let ni = ni as usize;
                let nj = nj as usize;

                if !visited[ni][nj] {
                    if let Some(next_node) = node.children.get_mut(&board[ni][nj]) {
                        backtrack(board, ni, nj, next_node, visited, result, directions);
                    }
                }
            }
        }

        visited[i][j] = false;
    }

    for i in 0..rows {
        for j in 0..cols {
            if let Some(node) = root.children.get_mut(&board[i][j]) {
                backtrack(&board, i, j, node, &mut visited, &mut result, &directions);
            }
        }
    }

    result.into_iter().collect()
}
