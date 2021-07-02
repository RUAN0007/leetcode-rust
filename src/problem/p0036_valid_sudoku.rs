/**
 * [36] Valid Sudoku
 *
 * Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
 * <ol>
 * 	Each row must contain the digits 1-9 without repetition.
 * 	Each column must contain the digits 1-9 without repetition.
 * 	Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
 * </ol>
 * Note:
 * 
 * 	A Sudoku board (partially filled) could be valid but is not necessarily solvable.
 * 	Only the filled cells need to be validated according to the mentioned rules.
 * 
 *  
 * Example 1:
 * <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png" style="height:250px; width:250px" />
 * Input: board = 
 * [["5","3",".",".","7",".",".",".","."]
 * ,["6",".",".","1","9","5",".",".","."]
 * ,[".","9","8",".",".",".",".","6","."]
 * ,["8",".",".",".","6",".",".",".","3"]
 * ,["4",".",".","8",".","3",".",".","1"]
 * ,["7",".",".",".","2",".",".",".","6"]
 * ,[".","6",".",".",".",".","2","8","."]
 * ,[".",".",".","4","1","9",".",".","5"]
 * ,[".",".",".",".","8",".",".","7","9"]]
 * Output: true
 * 
 * Example 2:
 * 
 * Input: board = 
 * [["8","3",".",".","7",".",".",".","."]
 * ,["6",".",".","1","9","5",".",".","."]
 * ,[".","9","8",".",".",".",".","6","."]
 * ,["8",".",".",".","6",".",".",".","3"]
 * ,["4",".",".","8",".","3",".",".","1"]
 * ,["7",".",".",".","2",".",".",".","6"]
 * ,[".","6",".",".",".",".","2","8","."]
 * ,[".",".",".","4","1","9",".",".","5"]
 * ,[".",".",".",".","8",".",".","7","9"]]
 * Output: false
 * Explanation: Same as Example 1, except with the 5 in the top left corner being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.
 * 
 *  
 * Constraints:
 * 
 * 	board.length == 9
 * 	board[i].length == 9
 * 	board[i][j] is a digit or '.'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-sudoku/
// discuss: https://leetcode.com/problems/valid-sudoku/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            let mut presence : HashSet<char> = HashSet::new();
            for j in 0..9 {
                if board[i][j] == '.' {continue;}
                if presence.contains(&board[i][j]) {
                    return false;
                }
                presence.insert(board[i][j]);
            }
        }

        for j in 0..9 {
            let mut presence : HashSet<char> = HashSet::new();
            for i in 0..9 {
                if board[i][j] == '.' {continue;}
                if presence.contains(&board[i][j]) {
                    return false;
                }
                presence.insert(board[i][j]);
            }
        }

        for row_group in 0..3 {
            for col_group in 0..3 {
                let mut presence : HashSet<char> = HashSet::new();
                for i in (row_group * 3)..((row_group+1)*3) {
                    for j in (col_group * 3)..((col_group+1)*3) {
                        if board[i][j] == '.' {continue;}
                        if presence.contains(&board[i][j]) {
                            return false;
                        }
                        presence.insert(board[i][j]);
                    }
                }
            }
        }

        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_36() {
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ]),
            false
        );
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            true
        );
    }
}
