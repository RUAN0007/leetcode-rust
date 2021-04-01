/**
 * [885] Spiral Matrix III
 *
 * On a 2 dimensional grid with R rows and C columns, we start at (r0, c0) facing east.
 * 
 * Here, the north-west corner of the grid is at the first row and column, and the south-east corner of the grid is at the last row and column.
 * 
 * Now, we walk in a clockwise spiral shape to visit every position in this grid. 
 * 
 * Whenever we would move outside the boundary of the grid, we continue our walk outside the grid (but may return to the grid boundary later.) 
 * 
 * Eventually, we reach all R * C spaces of the grid.
 * 
 * Return a list of coordinates representing the positions of the grid in the order they were visited.
 * 
 *  
 * 
 * Example 1:
 * 
 * 
 * Input: R = <span id="example-input-1-1">1</span>, C = <span id="example-input-1-2">4</span>, r0 = <span id="example-input-1-3">0</span>, c0 = <span id="example-input-1-4">0</span>
 * Output: <span id="example-output-1">[[0,0],[0,1],[0,2],[0,3]]</span>
 * 
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/08/24/example_1.png" style="width: 174px; height: 99px;" />
 * 
 * 
 *  
 * 
 * Example 2:
 * 
 * 
 * Input: R = <span id="example-input-2-1">5</span>, C = <span id="example-input-2-2">6</span>, r0 = <span id="example-input-2-3">1</span>, c0 = <span id="example-input-2-4">4</span>
 * Output: <span id="example-output-2">[[1,4],[1,5],[2,5],[2,4],[2,3],[1,3],[0,3],[0,4],[0,5],[3,5],[3,4],[3,3],[3,2],[2,2],[1,2],[0,2],[4,5],[4,4],[4,3],[4,2],[4,1],[3,1],[2,1],[1,1],[0,1],[4,0],[3,0],[2,0],[1,0],[0,0]]</span>
 * 
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/08/24/example_2.png" style="width: 202px; height: 142px;" />
 * 
 * 
 * <div>
 * <div>
 *  
 * 
 * Note:
 * 
 * <ol>
 * 	1 <= R <= 100
 * 	1 <= C <= 100
 * 	0 <= r0 < R
 * 	0 <= c0 < C
 * </ol>
 * </div>
 * </div>
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/spiral-matrix-iii/
// discuss: https://leetcode.com/problems/spiral-matrix-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn valid(r: i32, c: i32, r0: i32, c0: i32) -> bool {
        return 0 <= r0 && r0 < r && 0 <= c0 && c0 < c;
    }

    pub fn spiral_matrix_iii(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut cur_row_idx = r0;
        let mut cur_col_idx = c0;

        let mut direction_step = 1usize;
        'outer: loop {
            // move right
            for j in 0..direction_step {
                if Self::valid(r, c, cur_row_idx, cur_col_idx) {
                    result.push(vec![cur_row_idx, cur_col_idx]);
                    if result.len() as i32 == r * c {break 'outer}
                }
                cur_col_idx += 1;
            }

            // move down
            for i in 0..direction_step {
                if Self::valid(r, c, cur_row_idx, cur_col_idx) {
                    result.push(vec![cur_row_idx, cur_col_idx]);
                    if result.len() as i32 == r * c {break 'outer}
                }
                cur_row_idx += 1;
            }

            direction_step += 1;

            // move left
            for j in 0..direction_step {
                if Self::valid(r, c, cur_row_idx, cur_col_idx) {
                    result.push(vec![cur_row_idx, cur_col_idx]);
                    if result.len() as i32 == r * c {break 'outer}
                }
                cur_col_idx -= 1;
            }

            // move up
            for i in 0..direction_step {
                if Self::valid(r, c, cur_row_idx, cur_col_idx) {
                    result.push(vec![cur_row_idx, cur_col_idx]);
                    if result.len() as i32 == r * c {break 'outer}
                }
                cur_row_idx -= 1;

            }
            direction_step += 1;
        }


        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_885() {
        assert_eq!(Solution::spiral_matrix_iii(1, 4, 0, 0), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]]);

    }
}
