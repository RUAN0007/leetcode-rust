/**
 * [37] Sudoku Solver
 *
 * Write a program to solve a Sudoku puzzle by filling the empty cells.
 * A sudoku solution must satisfy all of the following rules:
 * <ol>
 * 	Each of the digits 1-9 must occur exactly once in each row.
 * 	Each of the digits 1-9 must occur exactly once in each column.
 * 	Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
 * </ol>
 * The '.' character indicates empty cells.
 *  
 * Example 1:
 * <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png" style="height:250px; width:250px" />
 * Input: board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
 * Output: [["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
 * Explanation: The input board is shown above and the only valid solution is shown below:
 * <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/31/Sudoku-by-L2G-20050714_solution.svg/250px-Sudoku-by-L2G-20050714_solution.svg.png" style="height:250px; width:250px" />
 * 
 *  
 * Constraints:
 * 
 * 	board.length == 9
 * 	board[i].length == 9
 * 	board[i][j] is a digit or '.'.
 * 	It is guaranteed that the input board has only one solution.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sudoku-solver/
// discuss: https://leetcode.com/problems/sudoku-solver/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_valid(board: &Vec<Vec<char>>, i : usize, j : usize, try_char : char) -> bool {
        for col in 0..9 {
            if board[i][col] == try_char {return false;}
        }
        for row in 0..9 {
            if board[row][j] == try_char {return false;}
        }

        let row_group_idx = i / 3;
        let col_group_idx = j / 3;
        for ii in (3 * row_group_idx)..(3*(row_group_idx+1)) {
            for jj in (3 * col_group_idx)..(3*(col_group_idx+1)) {
                if board[ii][jj] == try_char {return false;}
            }
        }
        true
    }

    pub fn recursive_solve(board: &mut Vec<Vec<char>>, all_missed_cells : &Vec<(usize, usize)>, cur_idx : usize) -> bool {
        if cur_idx == all_missed_cells.len() {
            return true;
        }

        let i : usize = all_missed_cells[cur_idx].0;
        let j : usize = all_missed_cells[cur_idx].1;

        for &c in ['1','2','3','4','5','6','7','8','9'].iter() {
            if !Self::is_valid(board, i,j,c) {continue};
            board[i][j] = c;
            if Self::recursive_solve(board, all_missed_cells, cur_idx + 1) {return true;}
            // This step is compulsory: when all options fail, it resets before backtracking. 
            board[i][j] = '.'; 
        }
        false
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut all_missed_cells : Vec<(usize, usize)> = vec![];
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    all_missed_cells.push((i,j));
                }
            }
        }
        Self::recursive_solve(board, &all_missed_cells, 0);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_37() {
        let mut board : Vec<Vec<char>> =  vec![vec!['5','3','.','.','7','.','.','.','.'],vec!['6','.','.','1','9','5','.','.','.'],vec!['.','9','8','.','.','.','.','6','.'],vec!['8','.','.','.','6','.','.','.','3'],vec!['4','.','.','8','.','3','.','.','1'],vec!['7','.','.','.','2','.','.','.','6'],vec!['.','6','.','.','.','.','2','8','.'],vec!['.','.','.','4','1','9','.','.','5'],vec!['.','.','.','.','8','.','.','7','9']];

        let output : Vec<Vec<char>> = vec![vec!['5','3','4','6','7','8','9','1','2'],vec!['6','7','2','1','9','5','3','4','8'],vec!['1','9','8','3','4','2','5','6','7'],vec!['8','5','9','7','6','1','4','2','3'],vec!['4','2','6','8','5','3','7','9','1'],vec!['7','1','3','9','2','4','8','5','6'],vec!['9','6','1','5','3','7','2','8','4'],vec!['2','8','7','4','1','9','6','3','5'],vec!['3','4','5','2','8','6','1','7','9']];

        Solution::solve_sudoku(&mut board);
        assert_eq!(board, output);
        
    }
}
