/**
 * [74] Search a 2D Matrix
 *
 * Write an efficient algorithm that searches for a value in an m x n matrix. This matrix has the following properties:
 * 
 * 	Integers in each row are sorted from left to right.
 * 	The first integer of each row is greater than the last integer of the previous row.
 * 
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/05/mat.jpg" style="width: 322px; height: 242px;" />
 * Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
 * Output: true
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/05/mat2.jpg" style="width: 322px; height: 242px;" />
 * Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 100
 * 	-10^4 <= matrix[i][j], target <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/search-a-2d-matrix/
// discuss: https://leetcode.com/problems/search-a-2d-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get(matrix: &Vec<Vec<i32>>, idx: usize) -> i32 {
       let col_count = matrix[0].len(); 
       let row_idx = idx / col_count;
       let col_idx = idx % col_count;
       matrix[row_idx as usize][col_idx as usize]
    }

    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let count = matrix.len() * matrix[0].len();
        let mut start = 0; // inclusive
        let mut end = count as usize; // exclusive

        while start < end {
            let mid = (start + end) / 2;
            let mid_num = Self::get(&matrix, mid as usize);
            if mid_num == target {
                return true;
            // } else if start == end - 1 {
            //     return false;
            } else if mid_num < target {
                start = mid+1;
            } else {
                end = mid;
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
    fn test_74() {
    }
}
