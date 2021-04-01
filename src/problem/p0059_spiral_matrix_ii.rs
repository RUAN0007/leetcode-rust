/**
 * [59] Spiral Matrix II
 *
 * Given a positive integer n, generate an n x n matrix filled with elements from 1 to n^2 in spiral order.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiraln.jpg" style="width: 242px; height: 242px;" />
 * Input: n = 3
 * Output: [[1,2,3],[8,9,4],[7,6,5]]
 * 
 * Example 2:
 * 
 * Input: n = 1
 * Output: [[1]]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 20
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/spiral-matrix-ii/
// discuss: https://leetcode.com/problems/spiral-matrix-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0;n as usize];n as usize];
        let row_count = n;
        let col_count = n;

        let mut top = 0i32;
        let mut bottom = (row_count - 1) as i32;
        let mut left = 0i32;
        let mut right = (col_count - 1) as i32;

        let mut num = 1;

        loop { 
            // println!("A top={}, bottom={}, left={}, right={}", top, bottom, left, right);
            for j in left..=right {
                matrix[top as usize][j as usize] = num;
                num += 1;
            }
            top += 1;
            if left > right || top > bottom {
                break;
            }

            // println!("B top={}, bottom={}, left={}, right={}", top, bottom, left, right);
            for i in top..=bottom {
                matrix[i as usize][right as usize] = num;
                num += 1;
            }
            right-=1;
            if left > right || top > bottom {break}

            // println!("C top={}, bottom={}, left={}, right={}", top, bottom, left, right);
            for j in (left..=right).rev() {
                matrix[bottom as usize][j as usize] = num;
                num += 1;
            }
            bottom -=1;
            if left > right || top > bottom {break}

            // println!("D top={}, bottom={}, left={}, right={}", top, bottom, left, right);
            for i in (top..=bottom).rev() {
                matrix[i as usize][left as usize] = num;
                num +=1 ;
            }
            left+=1;
            if left > right || top > bottom {break}
        }
        matrix
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_59() {
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
        assert_eq!(Solution::generate_matrix(2), vec![vec![1, 2], vec![4, 3]]);
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5],]
        );
    }
}
