/**
 * [498] Diagonal Traverse
 *
 * Given an m x n matrix mat, return an array of all the elements of the array in a diagonal order.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/10/diag1-grid.jpg" style="width: 334px; height: 334px;" />
 * Input: mat = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: [1,2,4,7,5,3,6,8,9]
 * 
 * Example 2:
 * 
 * Input: mat = [[1,2],[3,4]]
 * Output: [1,2,3,4]
 * 
 *  
 * Constraints:
 * 
 * 	m == mat.length
 * 	n == mat[i].length
 * 	1 <= m, n <= 10^4
 * 	1 <= m * n <= 10^4
 * 	-10^5 <= mat[i][j] <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/diagonal-traverse/
// discuss: https://leetcode.com/problems/diagonal-traverse/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn valid(i : i32, j : i32, row_count : i32, col_count : i32) -> bool {
        0 <= i && i < row_count && 0 <= j && j < col_count
    }
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        let row_count : i32 = mat.len() as i32;
        let col_count : i32 = mat[0].len() as i32;
        let mut i : i32 = 0;
        let mut j : i32 = 0;
        loop {
            while !Self::valid(i, j, row_count, col_count) {
                i-=1;
                j+=1;
            }
            while Self::valid(i, j, row_count, col_count) {
                result.push(mat[i as usize][j as usize]);
                i-=1;
                j+=1;
            }
            j+=1;
            if result.len() as i32 == row_count * col_count {
                break;
            }

            while !Self::valid(i, j, row_count, col_count) {
                i+=1;
                j-=1;
            }
            while Self::valid(i, j, row_count, col_count) {
                result.push(mat[i as usize][j as usize]);
                i+=1;
                j-=1;
            }
            j+=1;
            if result.len() as i32 == row_count * col_count {
                break;
            }

        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_498() {
        assert_eq!(Solution::find_diagonal_order(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]), vec![1,2,4,7,5,3,6,8,9]);
    }
}
