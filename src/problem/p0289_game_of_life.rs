/**
 * [289] Game of Life
 *
 * According to <a href="https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life" target="_blank">Wikipedia's article</a>: "The Game of Life, also known simply as Life, is a cellular automaton devised by the British mathematician John Horton Conway in 1970."
 * The board is made up of an m x n grid of cells, where each cell has an initial state: live (represented by a 1) or dead (represented by a 0). Each cell interacts with its <a href="https://en.wikipedia.org/wiki/Moore_neighborhood" target="_blank">eight neighbors</a> (horizontal, vertical, diagonal) using the following four rules (taken from the above Wikipedia article):
 * <ol>
 * 	Any live cell with fewer than two live neighbors dies as if caused by under-population.
 * 	Any live cell with two or three live neighbors lives on to the next generation.
 * 	Any live cell with more than three live neighbors dies, as if by over-population.
 * 	Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
 * </ol>
 * <span>The next state is created by applying the above rules simultaneously to every cell in the current state, where births and deaths occur simultaneously. Given the current state of the m x n grid board, return the next state.</span>
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/26/grid1.jpg" style="width: 562px; height: 322px;" />
 * Input: board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]
 * Output: [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/26/grid2.jpg" style="width: 402px; height: 162px;" />
 * Input: board = [[1,1],[1,0]]
 * Output: [[1,1],[1,1]]
 * 
 *  
 * Constraints:
 * 
 * 	m == board.length
 * 	n == board[i].length
 * 	1 <= m, n <= 25
 * 	board[i][j] is 0 or 1.
 * 
 *  
 * Follow up:
 * 
 * 	Could you solve it in-place? Remember that the board needs to be updated simultaneously: You cannot update some cells first and then use their updated values to update other cells.
 * 	In this question, we represent the board using a 2D array. In principle, the board is infinite, which would cause problems when the active area encroaches upon the border of the array (i.e., live cells reach the border). How would you address these problems?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/game-of-life/
// discuss: https://leetcode.com/problems/game-of-life/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let row_count : usize = board.len();            
        let col_count : usize = board[0].len();            

        for i in 0..row_count {
            for j in 0..col_count {
                let mut live_count : usize = 0;
                if 1 <= i && 1 <= j && board[i-1][j-1] & 0b1 == 1 {
                    live_count+=1; // left-up
                }

                if 1 <= i && board[i-1][j] & 0b1 == 1 {
                    live_count+=1; // up
                }

                if 1 <= i && j < col_count - 1 && board[i-1][j+1] & 0b1 == 1{
                    live_count +=1; // right up
                }

                if 1 <= j && board[i][j-1] & 0b1 == 1 {
                    live_count +=1; // left 
                }

                if j < col_count - 1 && board[i][j+1] & 0b1 == 1 {
                    live_count +=1; // right
                }

                if  i < row_count - 1 && 1 <= j && board[i+1][j-1] & 0b1 == 1 {
                    live_count+=1; // left-bottom
                }

                if  i < row_count - 1 && board[i+1][j] & 0b1 == 1 {
                    live_count+=1; // bottom
                }

                if  i < row_count - 1 && j < col_count - 1 && board[i+1][j+1] & 0b1 == 1{
                    live_count +=1; // right bottom
                }

                // println!("i={},j={},live_count={}",i,j,live_count);
                if board[i][j] == 1 {
                    if live_count == 2 || live_count == 3 {
                        board[i][j] |= 0b10;
                    }
                } else if live_count == 3 {
                        board[i][j] |= 0b10;
                }
            }
        }

        for i in 0..row_count {
            for j in 0..col_count {
                if board[i][j] & 0b10 != 0 {
                    board[i][j] = 1;
                } else {
                    board[i][j] = 0;
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_289() {
        let mut test = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        Solution::game_of_life(&mut test);
        assert_eq!(
            test,
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0],]
        );
    }
}
