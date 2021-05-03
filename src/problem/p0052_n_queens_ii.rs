/**
 * [52] N-Queens II
 *
 * The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.
 * Given an integer n, return the number of distinct solutions to the n-queens puzzle.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/queens.jpg" style="width: 600px; height: 268px;" />
 * Input: n = 4
 * Output: 2
 * Explanation: There are two distinct solutions to the 4-queens puzzle as shown.
 * 
 * Example 2:
 * 
 * Input: n = 1
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/n-queens-ii/
// discuss: https://leetcode.com/problems/n-queens-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn recursive_try(board : &mut Vec<Vec<bool>>, row_idx : usize) -> usize {
        let size : usize = board.len();
        if row_idx == size {return 1;}
        let mut sol_count : usize = 0;
        for col_idx in 0..size {
            let mut valid = true;
            // check the validity at (row_idx, col_idx)
            let mut i = 0;

            while valid && i < row_idx {
                // Same column in previous row
                if board[i][col_idx] {valid = false;}

                // Same / diagonal in previous row
                let j : i32 = row_idx as i32 + col_idx as i32 - i as i32;
                if 0 <= j && j < size as i32 {
                    if board[i][j as usize] {valid = false;}
                }

                // Same \ diagonal in previous row :
                //   row_idx - col_idx = i - j;
                let j : i32 = i as i32 + col_idx as i32 - row_idx as i32;
                if 0 <= j && j < size as i32  {
                    if board[i][j as usize] {valid = false;}
                }
                i+=1;
            }

            if valid {
                board[row_idx][col_idx] = true;
                sol_count += Self::recursive_try(board, row_idx + 1);
                board[row_idx][col_idx] = false;
            }
        }
        sol_count
    }

    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut board : Vec<Vec<bool>> = vec![vec![false; n];n];
        Self::recursive_try(&mut board, 0) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_52() {
        assert_eq!(Solution::total_n_queens(4), 2);
        assert_eq!(Solution::total_n_queens(8), 92);
        // assert_eq!(Solution::total_n_queens(13), 73712);
        // assert_eq!(Solution::total_n_queens(14), 365596);
    }
}
