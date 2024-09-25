// 212. Word Search II

// Given an m x n board of characters and a list of strings words, return all words on the board.

// Each word must be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.

 

// Example 1:


// Input: board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain"]
// Output: ["eat","oath"]
// Example 2:


// Input: board = [["a","b"],["c","d"]], words = ["abcb"]
// Output: []
 

// Constraints:

// m == board.length
// n == board[i].length
// 1 <= m, n <= 12
// board[i][j] is a lowercase English letter.
// 1 <= words.length <= 3 * 104
// 1 <= words[i].length <= 10
// words[i] consists of lowercase English letters.
// All the strings of words are unique.

use std::collections::HashSet;

#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 26],
    is_end: bool,
    idx: i32,
}

impl Node {
    fn new() -> Self {
        Node {
            children: Default::default(),
            is_end: false,
            idx: -1,
        }
    }

    fn is_present(&self, ch: char) -> bool {
        self.children[(ch as usize - 'a' as usize)].is_some()
    }

    fn create_child(&mut self, ch: char) {
        let idx = ch as usize - 'a' as usize;
        self.children[idx] = Some(Box::new(Node::new()));
    }

    fn move_to_child(&self, ch: char) -> Option<&Box<Node>> {
        let idx = ch as usize - 'a' as usize;
        self.children[idx].as_ref()
    }

    fn set_flag(&mut self, idx: i32) {
        self.is_end = true;
        self.idx = idx;
    }

    fn get_flag(&self) -> bool {
        self.is_end
    }

    fn get_idx(&self) -> i32 {
        self.idx
    }
}

struct Trie {
    dr: [i32; 4],
    dc: [i32; 4],
    root: Node,
}

impl Trie {
    fn new() -> Self {
        Trie {
            dr: [-1, 0, 1, 0],
            dc: [0, 1, 0, -1],
            root: Node::new(),
        }
    }

    fn insert(&mut self, word: &str, idx: i32) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            if !node.is_present(ch) {
                node.create_child(ch);
            }
            node = node.move_to_child(ch).unwrap();
        }
        node.set_flag(idx);
    }

    fn solve_util(
        &self,
        node: &Node,
        row: usize,
        col: usize,
        board: &Vec<Vec<char>>,
        n: usize,
        m: usize,
        inc: &mut Vec<i32>,
        vis: &mut Vec<Vec<bool>>,
    ) {
        if node.get_flag() && inc[node.get_idx() as usize] == 0 {
            inc[node.get_idx() as usize] = 1;
        }

        vis[row][col] = true;

        for i in 0..4 {
            let nrow = (row as i32 + self.dr[i]) as usize;
            let ncol = (col as i32 + self.dc[i]) as usize;

            if nrow < n && ncol < m && !vis[nrow][ncol] {
                if let Some(child_node) = node.move_to_child(board[nrow][ncol]) {
                    self.solve_util(child_node, nrow, ncol, board, n, m, inc, vis);
                }
            }
        }

        vis[row][col] = false;
    }

    fn solve(&self, board: &Vec<Vec<char>>, n: usize, m: usize, inc: &mut Vec<i32>) {
        let mut vis = vec![vec![false; m]; n];
        for i in 0..n {
            for j in 0..m {
                if self.root.is_present(board[i][j]) {
                    self.solve_util(
                        self.root.move_to_child(board[i][j]).unwrap(),
                        i,
                        j,
                        board,
                        n,
                        m,
                        inc,
                        &mut vis,
                    );
                }
            }
        }
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();

        for (i, word) in words.iter().enumerate() {
            trie.insert(word, i as i32);
        }

        let mut inc = vec![0; words.len()];
        let n = board.len();
        let m = board[0].len();

        trie.solve(&board, n, m, &mut inc);

        words.into_iter()
            .enumerate()
            .filter_map(|(i, word)| if inc[i] == 1 { Some(word) } else { None })
            .collect()
    }
}
