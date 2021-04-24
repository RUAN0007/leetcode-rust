use serde::de::Visitor;

/**
 * [79] Word Search
 *
 * Given an m x n grid of characters board and a string word, return true if word exists in the grid.
 * The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/word2.jpg" style="width: 322px; height: 242px;" />
 * Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
 * Output: true
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/word-1.jpg" style="width: 322px; height: 242px;" />
 * Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
 * Output: true
 * 
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/15/word3.jpg" style="width: 322px; height: 242px;" />
 * Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	m == board.length
 * 	n = board[i].length
 * 	1 <= m, n <= 6
 * 	1 <= word.length <= 15
 * 	board and word consists of only lowercase and uppercase English letters.
 * 
 *  
 * Follow up: Could you use search pruning to make your solution faster with a larger board?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-search/
// discuss: https://leetcode.com/problems/word-search/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashSet;
impl Solution {
    pub fn track(board: &Vec<Vec<char>>, i : i32, j : i32, target : &String, visited: &mut HashSet<(i32,i32)>) -> bool{
        if target.len() == 0 {return true;}
        let target_char : char = target.chars().nth(0).unwrap();
        let row_count : i32 = board.len() as i32;
        let col_count : i32 = board[0].len() as i32;
        let mut found = false;
        if (!visited.contains(&(i,j)) && 0 <= i  && i < row_count && 0 <= j && j < col_count && board[i as usize][j as usize] == target_char) {
            let next_target : String = String::from(&target[1..]);
            visited.insert((i,j));
            found = Self::track(board, i-1, j, &next_target, visited) ||
            Self::track(board, i+1, j, &next_target, visited) ||
            Self::track(board, i, j-1, &next_target, visited) ||
            Self::track(board, i, j+1, &next_target, visited);
            visited.remove(&(i,j));
        }

        found
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let mut visited = HashSet::new();
                if Self::track(&board, i as i32, j as i32, &word, &mut visited) {return true;}
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_79() {
        assert_eq!(Solution::exist(vec![vec!['a']], "a".to_owned()), true);
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCCED".to_owned()
            ),
            true
        );
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "SEE".to_owned()
            ),
            true
        );
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCB".to_owned()
            ),
            false
        );
    }
}
