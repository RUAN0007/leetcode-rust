/**
 * [64] Minimum Path Sum
 *
 * Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right, which minimizes the sum of all numbers along its path.
 * Note: You can only move either down or right at any point in time.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/05/minpath.jpg" style="width: 242px; height: 242px;" />
 * Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
 * Output: 7
 * Explanation: Because the path 1 &rarr; 3 &rarr; 1 &rarr; 1 &rarr; 1 minimizes the sum.
 * 
 * Example 2:
 * 
 * Input: grid = [[1,2,3],[4,5,6]]
 * Output: 12
 * 
 *  
 * Constraints:
 * 
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 200
 * 	0 <= grid[i][j] <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-path-sum/
// discuss: https://leetcode.com/problems/minimum-path-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let depth = grid.len();
        let width = grid[0].len();
        let mut sum = grid.clone();  

        for i in 0..depth {
            for j in 0..width {
                if 0 < i && 0 < j{
                    sum[i][j] = std::cmp::min(sum[i-1][j], sum[i][j-1]) + grid[i][j];
                } else if 0 < i {
                    sum[i][j] = sum[i-1][j] + grid[i][j];
                } else if 0 < j {
                    sum[i][j] = sum[i][j-1] + grid[i][j];
                } else {
                    sum[i][j] = grid[i][j];
                }
            }
        }
        sum[depth-1][width-1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_64() {
        assert_eq!(Solution::min_path_sum(vec![vec![2]]), 2);
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1],]),
            7
        );
        assert_eq!(Solution::min_path_sum(vec![vec![1, 3, 1],]), 5);
    }
}
