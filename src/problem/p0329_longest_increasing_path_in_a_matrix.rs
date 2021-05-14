/**
 * [329] Longest Increasing Path in a Matrix
 *
 * Given an m x n integers matrix, return the length of the longest increasing path in matrix.
 * From each cell, you can either move in four directions: left, right, up, or down. You may not move diagonally or move outside the boundary (i.e., wrap-around is not allowed).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/05/grid1.jpg" style="width: 242px; height: 242px;" />
 * Input: matrix = [[9,9,4],[6,6,8],[2,1,1]]
 * Output: 4
 * Explanation: The longest increasing path is [1, 2, 6, 9].
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/27/tmp-grid.jpg" style="width: 253px; height: 253px;" />
 * Input: matrix = [[3,4,5],[3,2,6],[2,2,1]]
 * Output: 4
 * Explanation: The longest increasing path is [3, 4, 5, 6]. Moving diagonally is not allowed.
 * 
 * Example 3:
 * 
 * Input: matrix = [[1]]
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 200
 * 	0 <= matrix[i][j] <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-increasing-path-in-a-matrix/
// discuss: https://leetcode.com/problems/longest-increasing-path-in-a-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn compute(all_longest : &mut Vec<Vec<i32>>, matrix : &Vec<Vec<i32>>, i : usize, j : usize) -> i32 {
        if all_longest[i][j] != -1 {
            return all_longest[i][j];
        }

        let row_count : usize = all_longest.len();
        let col_count : usize = all_longest[0].len();
        let mut nearby_max_longest : i32 = 0;

        if 0 < i && matrix[i-1][j] < matrix[i][j] {
            nearby_max_longest = std::cmp::max(nearby_max_longest, Self::compute(all_longest, matrix, i - 1, j));
        }

        if i < row_count - 1 && matrix[i+1][j] < matrix[i][j] {
            nearby_max_longest = std::cmp::max(nearby_max_longest, Self::compute(all_longest, matrix, i + 1, j));
        }

        if 0 < j && matrix[i][j-1] < matrix[i][j] {
            nearby_max_longest = std::cmp::max(nearby_max_longest, Self::compute(all_longest, matrix, i, j-1));
        }

        if j < col_count - 1 && matrix[i][j+1] < matrix[i][j] {
            nearby_max_longest = std::cmp::max(nearby_max_longest, Self::compute(all_longest, matrix, i, j+1));
        }
        all_longest[i][j] = nearby_max_longest + 1;
        return all_longest[i][j];
    }

    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let row_count : usize = matrix.len();
        let col_count : usize = matrix[0].len();
        let mut all_longest : Vec<Vec<i32>> = vec![vec![-1;col_count];row_count];
        let mut longest : i32 = 0;
        for i in 0..row_count {
            for j in 0..col_count {
                Self::compute(&mut all_longest, &matrix, i, j);
                longest = std::cmp::max(longest, all_longest[i][j]);
            }
        }
        longest
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_329() {
        assert_eq!(Solution::longest_increasing_path( vec![vec![9,9,4],vec![6,6,8],vec![2,1,1]]), 4);
    }
}
