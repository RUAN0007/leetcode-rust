/**
 * [85] Maximal Rectangle
 *
 * Given a rows x cols binary matrix filled with 0's and 1's, find the largest rectangle containing only 1's and return its area.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/14/maximal.jpg" style="width: 402px; height: 322px;" />
 * Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
 * Output: 6
 * Explanation: The maximal rectangle is shown in the above picture.
 * 
 * Example 2:
 * 
 * Input: matrix = []
 * Output: 0
 * 
 * Example 3:
 * 
 * Input: matrix = [["0"]]
 * Output: 0
 * 
 * Example 4:
 * 
 * Input: matrix = [["1"]]
 * Output: 1
 * 
 * Example 5:
 * 
 * Input: matrix = [["0","0"]]
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	rows == matrix.length
 * 	cols == matrix[i].length
 * 	0 <= row, cols <= 200
 * 	matrix[i][j] is '0' or '1'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximal-rectangle/
// discuss: https://leetcode.com/problems/maximal-rectangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let row_count : usize = matrix.len();
        if row_count == 0 {return 0;}
        let col_count : usize = matrix[0].len();
        let mut heights : Vec<Vec<usize>> = vec![vec![0;col_count];row_count];
        let mut cur_max : i32 = 0;

        for i in 0..row_count {
            for j in 0..col_count {
                if matrix[i][j] != '1' {continue;}
                heights[i][j] = 1;
                if i >= 1 {heights[i][j] += heights[i-1][j] }
            }

            let mut left_stack : Vec<usize> = vec![];
            let mut leftmost_shorter : Vec<i32> = vec![];
            for j in 0..col_count {
                while left_stack.len() != 0 {
                    let prev_col : usize = *left_stack.last().unwrap();
                    if heights[i][prev_col] < heights[i][j] {
                        break
                    } else {
                        left_stack.pop();
                    }
                }
                if left_stack.len() == 0 {
                    leftmost_shorter.push(-1);
                } else {
                    leftmost_shorter.push(*left_stack.last().unwrap() as i32);
                }
                left_stack.push(j);
            }

            let mut right_stack : Vec<usize> = vec![];
            let mut rightmost_shorter : Vec<i32> = vec![];
            for j in (0..col_count).rev() {
                while right_stack.len() != 0 {
                    let prev_col : usize = *right_stack.last().unwrap();
                    if heights[i][prev_col] < heights[i][j] {
                        break
                    } else {
                        right_stack.pop();
                    }
                }
                if right_stack.len() == 0 {
                    rightmost_shorter.push(col_count as i32);
                } else {
                    rightmost_shorter.push(*right_stack.last().unwrap() as i32);
                }
                right_stack.push(j);
            }
            rightmost_shorter = rightmost_shorter.into_iter().rev().collect();

            // println!("i={}", i);
            // println!("heights={:?}", heights[i]);
            // println!("leftmost_shorter={:?}", leftmost_shorter);
            // println!("rightmost_shorter={:?}", rightmost_shorter);
            for j in 0..col_count {
                cur_max = std::cmp::max(cur_max, (rightmost_shorter[j] - leftmost_shorter[j] - 1) * heights[i][j] as i32);
            }
        }
        cur_max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_85() {
        assert_eq!(Solution::maximal_rectangle(
            vec![vec!['1','0','1','0','0'],vec!['1','0','1','1','1'],vec!['1','1','1','1','1'],vec!['1','0','0','1','0']]), 6);
    }
}
