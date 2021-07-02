/**
 * [240] Search a 2D Matrix II
 *
 * Write an efficient algorithm that searches for a target value in an m x n integer matrix. The matrix has the following properties:
 * 
 * 	Integers in each row are sorted in ascending from left to right.
 * 	Integers in each column are sorted in ascending from top to bottom.
 * 
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/24/searchgrid2.jpg" style="width: 300px; height: 300px;" />
 * Input: matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 5
 * Output: true
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/24/searchgrid.jpg" style="width: 300px; height: 300px;" />
 * Input: matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 20
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= n, m <= 300
 * 	-10^9 <= matix[i][j] <= 10^9
 * 	All the integers in each row are sorted in ascending order.
 * 	All the integers in each column are sorted in ascending order.
 * 	-10^9 <= target <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/search-a-2d-matrix-ii/
// discuss: https://leetcode.com/problems/search-a-2d-matrix-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut i : i32 = 0;
        let mut j : i32 = matrix[0].len() as i32 - 1;
        let row_count : i32 = matrix.len() as i32;

        while i < row_count && j >= 0 {
            let num : i32 = matrix[i as usize][j as usize];
            if num == target {
                return true;
            } else if num > target {
                j-=1;
            } else {
                i+=1;
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
    fn test_240() {
        assert!(!Solution::search_matrix(vec![vec![-5]], -10));
    }
}
