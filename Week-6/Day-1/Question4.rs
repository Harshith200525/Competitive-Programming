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

use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for w in word.chars() {
            node = node.children.entry(w).or_default();
        }
        node.is_word = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for w in word.chars() {
            match node.children.get(&w) {
                Some(n) => node = n,
                None => return false,
            }
        }
        node.is_word
    }
}

struct Solution;

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut res = Vec::new();
        let mut trie = Trie::new();

        // Insert words into the Trie
        for word in &words {
            trie.insert(word);
        }

        let mut visited = vec![vec![false; board[0].len()]; board.len()];

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                self.dfs(&board, &mut trie.root, i, j, &mut String::new(), &mut res, &mut visited);
            }
        }

        res
    }

    fn dfs(
        &self,
        board: &[Vec<char>],
        node: &mut TrieNode,
        i: usize,
        j: usize,
        path: &mut String,
        res: &mut Vec<String>,
        visited: &mut Vec<Vec<bool>>,
    ) {
        if node.is_word {
            res.push(path.clone());
            node.is_word = false; // Avoid duplicate entries
        }

        if i < 0 || i >= board.len() || j < 0 || j >= board[0].len() || visited[i][j] {
            return;
        }

        let tmp = board[i][j];
        if let Some(next_node) = node.children.get(&tmp) {
            visited[i][j] = true;
            path.push(tmp);

            // Explore neighbors
            self.dfs(board, next_node, i + 1, j, path, res, visited);
            self.dfs(board, next_node, i, j + 1, path, res, visited);
            self.dfs(board, next_node, i.wrapping_sub(1), j, path, res, visited);
            self.dfs(board, next_node, i, j.wrapping_sub(1), path, res, visited);
            
            visited[i][j] = false; // Unmark for other paths
            path.pop(); // Remove last character after exploring
        }
    }
}
