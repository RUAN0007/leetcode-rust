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

impl Solution {
    pub fn visited(route: &mut Vec<(usize, usize)>, pos: (usize, usize)) -> bool {
        let (this_row, this_col) = pos;
        for &mut(prev_row, prev_col) in route {
            if prev_row == this_row && prev_col == this_col {
                return true;
            }
        }
        false
    }

    pub fn track(board: &Vec<Vec<char>>, route: &mut Vec<(usize, usize)>, word: &String, pos: usize) -> bool {
        if word.len() == pos {
            return true;
        }

        let target_char = word.chars().nth(pos).unwrap();
        let &(prev_row, prev_col) = route.last().unwrap();
        let mut potential_pos = vec![];

        // up
        if 0 < prev_row {
            let cur_row = prev_row - 1;
            let cur_col = prev_col;
            let cur_pos = (cur_row, cur_col);
            if board[cur_row][cur_col] == target_char && !Self::visited(route, cur_pos){
                potential_pos.push(cur_pos);
            }
        } 

        // left
        if 0 < prev_col {
            let cur_row = prev_row;
            let cur_col = prev_col - 1;
            let cur_pos = (cur_row, cur_col);
            if board[cur_row][cur_col] == target_char && !Self::visited(route, cur_pos){
                potential_pos.push(cur_pos);
            }
        }

        // down
        let row_count = board.len();
        if prev_row < row_count - 1 {
            let cur_row = prev_row + 1;
            let cur_col = prev_col;
            let cur_pos = (cur_row, cur_col);
            if board[cur_row][cur_col] == target_char && !Self::visited(route, cur_pos){
                potential_pos.push(cur_pos);
            }
        }

        // right
        let col_count = board[0].len();
        if prev_col < col_count - 1 {
            let cur_row = prev_row;
            let cur_col = prev_col + 1;
            let cur_pos = (cur_row, cur_col);
            if board[cur_row][cur_col] == target_char && !Self::visited(route, cur_pos){
                potential_pos.push(cur_pos);
            }
        }

        for (cur_row, cur_col) in potential_pos {
            route.push((cur_row, cur_col));
            if Self::track(board, route, word, pos+1) {
                return true;
            }
            route.pop();
        }

        false
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let first_char = word.chars().nth(0).unwrap();
        for (row_idx, row) in board.iter().enumerate() {
            for (col_idx, &this_char) in row.iter().enumerate() {
                if  this_char == first_char && Self::track(&board, &mut vec![(row_idx, col_idx)], &word, 1) {
                   return true; 
                }
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
