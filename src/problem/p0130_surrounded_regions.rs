use std::cell;

/**
 * [130] Surrounded Regions
 *
 * Given an m x n matrix board containing 'X' and 'O', capture all regions surrounded by 'X'.
 * A region is captured by flipping all 'O's into 'X's in that surrounded region.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/xogrid.jpg" style="width: 550px; height: 237px;" />
 * Input: board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
 * Output: [["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
 * Explanation: Surrounded regions should not be on the border, which means that any 'O' on the border of the board are not flipped to 'X'. Any 'O' that is not on the border and it is not connected to an 'O' on the border will be flipped to 'X'. Two cells are connected if they are adjacent cells connected horizontally or vertically.
 * 
 * Example 2:
 * 
 * Input: board = [["X"]]
 * Output: [["X"]]
 * 
 *  
 * Constraints:
 * 
 * 	m == board.length
 * 	n == board[i].length
 * 	1 <= m, n <= 200
 * 	board[i][j] is 'X' or 'O'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/surrounded-regions/
// discuss: https://leetcode.com/problems/surrounded-regions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_border_zeroes(board: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
        let mut result = vec![];
        let (row_count, col_count) = (board.len(), board[0].len());

        board.iter().enumerate().for_each(|(row_idx, row)| {
            row.iter().enumerate().for_each(|(col_idx, &cell_char)| {
                let on_border = (row_idx == 0 || row_idx == row_count - 1 || col_idx == 0 || col_idx == col_count - 1);
                if on_border && cell_char == 'O' {
                    result.push((row_idx, col_idx));
                }
            });
        });
        result
    }

    pub fn unsurrounded_zeroes(board: &Vec<Vec<char>>, cur_route : &mut Vec<(usize, usize)>, pos: (i32, i32) ) {
        let (row_count, col_count) = (board.len() as i32, board[0].len() as i32);
        let (row_idx, col_idx) = pos;

        if 0 <= row_idx &&  row_idx < row_count && 0 <= col_idx && col_idx < col_count && board[row_idx as usize][col_idx as usize] == 'O' && !cur_route.contains(&(row_idx as usize, col_idx as usize)) {
            cur_route.push((row_idx as usize, col_idx as usize));
            Self::unsurrounded_zeroes(board, cur_route, (row_idx-1, col_idx));
            Self::unsurrounded_zeroes(board, cur_route, (row_idx+1, col_idx));
            Self::unsurrounded_zeroes(board, cur_route, (row_idx, col_idx-1));
            Self::unsurrounded_zeroes(board, cur_route, (row_idx, col_idx+1));
        } 
    }

    pub fn solve(board: &mut Vec<Vec<char>>) {
        let border_zeroes = Self::find_border_zeroes(board);

        for start_pos in border_zeroes {
            let row_idx = start_pos.0 as i32;
            let col_idx = start_pos.1 as i32;
            let mut route = vec![start_pos];
            Self::unsurrounded_zeroes(board, &mut route, (row_idx-1, col_idx));
            Self::unsurrounded_zeroes(board, &mut route, (row_idx+1, col_idx));
            Self::unsurrounded_zeroes(board, &mut route, (row_idx, col_idx-1));
            Self::unsurrounded_zeroes(board, &mut route, (row_idx, col_idx+1));

            for (row_idx, col_idx) in route {
                board[row_idx][col_idx] = '1'; // tmp change to 1 to avoid revisited. 
            }
        }

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                } else if board[i][j] == '1' {
                    board[i][j] = 'O';
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
    fn test_130() {
        let mut matrix = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        );

        let mut matrix = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'X', 'X'],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
            ]
        );

        let mut matrix = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['O', 'X', 'O', 'X'],
            vec!['O', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['O', 'X', 'X', 'X'],
                vec!['O', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        );

        let mut matrix = vec![
            vec!['X', 'X', 'X', 'X', 'O', 'X'],
            vec!['O', 'X', 'X', 'O', 'O', 'X'],
            vec!['X', 'O', 'X', 'O', 'O', 'O'],
            vec!['X', 'O', 'O', 'O', 'X', 'O'],
            vec!['O', 'O', 'X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec!['X', 'X', 'X', 'X', 'O', 'X'],
                vec!['O', 'X', 'X', 'O', 'O', 'X'],
                vec!['X', 'O', 'X', 'O', 'O', 'O'],
                vec!['X', 'O', 'O', 'O', 'X', 'O'],
                vec!['O', 'O', 'X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'O', 'X', 'X'],
            ]
        );

        let mut matrix = vec![
            vec![
                'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'O', 'O', 'O', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'O', 'O', 'O', 'X', 'O', 'X', 'O', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'O', 'X', 'O', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'X', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![
                    'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'O', 'O', 'O', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'O', 'O', 'O', 'X', 'O', 'X', 'O', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'O', 'X', 'O', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'X', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ]
            ]
        );
    }
}
