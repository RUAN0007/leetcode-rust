/**
 * [54] Spiral Matrix
 *
 * Given an m x n matrix, return all elements of the matrix in spiral order.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiral1.jpg" style="width: 242px; height: 242px;" />
 * Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: [1,2,3,6,9,8,7,4,5]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiral.jpg" style="width: 322px; height: 242px;" />
 * Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
 * Output: [1,2,3,4,8,12,11,10,9,5,6,7]
 * 
 *  
 * Constraints:
 * 
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 10
 * 	-100 <= matrix[i][j] <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/spiral-matrix/
// discuss: https://leetcode.com/problems/spiral-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        let row_count = matrix.len();
        let col_count = matrix[0].len();

        let mut top = 0i32;
        let mut bottom = (row_count - 1) as i32;
        let mut left = 0i32;
        let mut right = (col_count - 1) as i32;

        loop { 
            // println!("A top={}, bottom={}, left={}, right={}", top, bottom, left, right);
            for j in left..=right {
                result.push(matrix[top as usize][j as usize]);
            }
            top += 1;
            if left > right || top > bottom {
                break;
            }

            // println!("B top={}, bottom={}, left={}, right={}", top, bottom, left, right);
            for i in top..=bottom {
                result.push(matrix[i as usize][right as usize]);
            }
            right-=1;
            if left > right || top > bottom {break}

            // println!("C top={}, bottom={}, left={}, right={}", top, bottom, left, right);
            for j in (left..=right).rev() {
                result.push(matrix[bottom as usize][j as usize]);
            }
            bottom -=1;
            if left > right || top > bottom {break}

            // println!("D top={}, bottom={}, left={}, right={}", top, bottom, left, right);
            for i in (top..=bottom).rev() {
                result.push(matrix[i as usize][left as usize]);
            }
            left+=1;
            if left > right || top > bottom {break}
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_54() {

        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(Solution::spiral_order(vec![vec![1, 2, 3]]), vec![1, 2, 3]);
        assert_eq!(
            Solution::spiral_order(vec![vec![1], vec![2], vec![3],]),
            vec![1, 2, 3]
        );
        assert_eq!(Solution::spiral_order(vec![vec![1],]), vec![1]);
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2], vec![4, 5],]),
            vec![1, 2, 5, 4]
        );
    }
}
