/**
 * [63] Unique Paths II
 *
 * A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
 * The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
 * Now consider if some obstacles are added to the grids. How many unique paths would there be?
 * An obstacle and space is marked as 1 and 0 respectively in the grid.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/robot1.jpg" style="width: 242px; height: 242px;" />
 * Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
 * Output: 2
 * Explanation: There is one obstacle in the middle of the 3x3 grid above.
 * There are two ways to reach the bottom-right corner:
 * 1. Right -> Right -> Down -> Down
 * 2. Down -> Down -> Right -> Right
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/robot2.jpg" style="width: 162px; height: 162px;" />
 * Input: obstacleGrid = [[0,1],[0,0]]
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	m == obstacleGrid.length
 * 	n == obstacleGrid[i].length
 * 	1 <= m, n <= 100
 * 	obstacleGrid[i][j] is 0 or 1.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-paths-ii/
// discuss: https://leetcode.com/problems/unique-paths-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let row_count : usize = obstacle_grid.len();
        let col_count : usize = obstacle_grid[0].len();
        if obstacle_grid[row_count-1][col_count-1] == 1 {
            return 0;
        }

        let mut path_count : Vec<Vec<i32>> = vec![vec![0;col_count];row_count];
        path_count[row_count-1][col_count-1] = 1;
        for i in (0..row_count).rev() {
            for j in (0..col_count).rev() {
                if i == row_count - 1 && j == col_count - 1 {continue}
                if obstacle_grid[i][j] == 1 {continue} 
                let mut right_path_count : i32 = 0;
                if j < col_count - 1 && obstacle_grid[i][j+1] == 0 {
                    right_path_count = path_count[i][j+1];
                }

                let mut down_path_count : i32 = 0;
                if i < row_count - 1 && obstacle_grid[i+1][j] == 0 {
                    down_path_count = path_count[i+1][j];
                }
                path_count[i][j] = down_path_count + right_path_count;
            }
        }
        path_count[0][0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_63() {
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0]]), 1);
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![0, 0],]),
            2
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![1, 0],]),
            0
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0],
            ]),
            2
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]),
            10
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 1],
                vec![0, 0, 1, 0],
            ]),
            0
        );
    }
}
