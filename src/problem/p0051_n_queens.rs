/**
 * [51] N-Queens
 *
 * The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.
 * Given an integer n, return all distinct solutions to the n-queens puzzle.
 * Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space, respectively.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/queens.jpg" style="width: 600px; height: 268px;" />
 * Input: n = 4
 * Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
 * Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above
 * 
 * Example 2:
 * 
 * Input: n = 1
 * Output: [["Q"]]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/n-queens/
// discuss: https://leetcode.com/problems/n-queens/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn helper(cur_assign: &mut Vec<(i32, i32)>, n : i32) -> Vec<Vec<String>> {
        // println!("Cur_assign = {:?}", cur_assign);
        let mut results = vec![];
        if cur_assign.len() == n as usize {
            // a valid assignment
            let mut result = vec![];
            for (row, col) in cur_assign.clone() {
                let mut pattern: Vec<char> = (0..n).map(|_| '.').collect();
                pattern[col as usize] = 'Q';
                result.push(pattern.into_iter().collect());
            }
            results.push(result);
        } else {
            for col in 0..n {
                let row = cur_assign.len() as i32;
                let mut valid = true;
                for (prev_row, prev_col) in cur_assign.clone() {
                    if prev_col == col {
                        // same column
                        valid = false;
                        continue;
                    } 
                    if prev_row - prev_col == row - col {
                        // same \\ diagonal
                        valid = false;
                        continue;
                    }
                    if prev_row + prev_col == row + col {
                        // same // diagonal
                        valid = false;
                        continue;
                    }
                }
                
                if valid {
                    cur_assign.push((row, col));
                    results.append(&mut Self::helper(cur_assign, n));
                    cur_assign.pop();
                }
            }
        }

        results
    }
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut pos = vec![];
        Self::helper(&mut pos, n)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_51() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
        assert_eq!(Solution::solve_n_queens(8).len(), 92);
    }
}
