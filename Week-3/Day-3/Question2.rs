// 947. Most Stones Removed with Same Row or Column

// On a 2D plane, we place n stones at some integer coordinate points. Each coordinate point may have at most one stone.

// A stone can be removed if it shares either the same row or the same column as another stone that has not been removed.

// Given an array stones of length n where stones[i] = [xi, yi] represents the location of the ith stone, return the largest possible number of stones that can be removed.

 

// Example 1:

// Input: stones = [[0,0],[0,1],[1,0],[1,2],[2,1],[2,2]]
// Output: 5
// Explanation: One way to remove 5 stones is as follows:
// 1. Remove stone [2,2] because it shares the same row as [2,1].
// 2. Remove stone [2,1] because it shares the same column as [0,1].
// 3. Remove stone [1,2] because it shares the same row as [1,0].
// 4. Remove stone [1,0] because it shares the same column as [0,0].
// 5. Remove stone [0,1] because it shares the same row as [0,0].
// Stone [0,0] cannot be removed since it does not share a row/column with another stone still on the plane.
// Example 2:

// Input: stones = [[0,0],[0,2],[1,1],[2,0],[2,2]]
// Output: 3
// Explanation: One way to make 3 moves is as follows:
// 1. Remove stone [2,2] because it shares the same row as [2,0].
// 2. Remove stone [2,0] because it shares the same column as [0,0].
// 3. Remove stone [0,2] because it shares the same row as [0,0].
// Stones [0,0] and [1,1] cannot be removed since they do not share a row/column with another stone still on the plane.
// Example 3:

// Input: stones = [[0,0]]
// Output: 0
// Explanation: [0,0] is the only stone on the plane, so you cannot remove it.
 

// Constraints:

// 1 <= stones.length <= 1000
// 0 <= xi, yi <= 104
// No two stones are at the same coordinate point.

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();
        let m = 10001;
        let size = 2 * m + 1;
        let mut root: Vec<usize> = (0..size).collect();
        let mut sizes: Vec<i32> = vec![1; size];
        let mut merge = 0;

        fn find(x: usize, root: &mut Vec<usize>) -> usize {
            if x == root[x] {
                x
            } else {
                root[x] = find(root[x], root);
                root[x]
            }
        }

        fn union(x: usize, y: usize, root: &mut Vec<usize>, sizes: &mut Vec<i32>, merge: &mut i32) -> bool {
            let x_root = find(x, root);
            let y_root = find(y, root);
            if x_root == y_root {
                return false;
            }
            if sizes[x_root] > sizes[y_root] {
                sizes[x_root] += sizes[y_root];
                root[y_root] = x_root;
            } else {
                sizes[y_root] += sizes[x_root];
                root[x_root] = y_root;
            }
            *merge += 1;
            true
        }

        let mut cnt_rc = vec![false; size];
        for stone in &stones {
            let i = stone[0] as usize;
            let j = stone[1] as usize;
            union(i, m + j, &mut root, &mut sizes, &mut merge);
            cnt_rc[i] = true;
            cnt_rc[m + j] = true;
        }

        let count_true = cnt_rc.iter().filter(|&&x| x).count();
        (n - count_true) as i32 + merge
    }
}
