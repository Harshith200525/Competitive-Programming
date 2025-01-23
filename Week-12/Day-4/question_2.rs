// Question 2: Word Search III
// How to efficiently navigate a grid with word search logic and additional constraints?

use std::collections::HashSet;

fn question_2(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    #[derive(Default)]
    struct TrieNode {
        children: std::collections::HashMap<char, TrieNode>,
        word: Option<String>,
    }

    let mut root = TrieNode::default();
    for word in words {
        let mut node = &mut root;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        node.word = Some(word);
    }

    let mut result = HashSet::new();
    let rows = board.len();
    let cols = board[0].len();
    let mut visited = vec![vec![false; cols]; rows];

    fn backtrack(
        x: usize,
        y: usize,
        node: &mut TrieNode,
        board: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        result: &mut HashSet<String>,
    ) {
        if let Some(word) = &node.word {
            result.insert(word.clone());
        }

        visited[x][y] = true;

        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && nx < board.len() as i32 && ny < board[0].len() as i32 {
                let nx = nx as usize;
                let ny = ny as usize;

                if !visited[nx][ny] {
                    if let Some(next_node) = node.children.get_mut(&board[nx][ny]) {
                        backtrack(nx, ny, next_node, board, visited, result);
                    }
                }
            }
        }

        visited[x][y] = false;
    }

    for i in 0..rows {
        for j in 0..cols {
            if let Some(node) = root.children.get_mut(&board[i][j]) {
                backtrack(i, j, node, &board, &mut visited, &mut result);
            }
        }
    }

    result.into_iter().collect()
}
