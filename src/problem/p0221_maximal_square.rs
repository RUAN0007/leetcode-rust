/**
 * [221] Maximal Square
 *
 * Given an m x n binary matrix filled with 0's and 1's, find the largest square containing only 1's and return its area.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/26/max1grid.jpg" style="width: 400px; height: 319px;" />
 * Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
 * Output: 4
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/26/max2grid.jpg" style="width: 165px; height: 165px;" />
 * Input: matrix = [["0","1"],["1","0"]]
 * Output: 1
 * 
 * Example 3:
 * 
 * Input: matrix = [["0"]]
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 300
 * 	matrix[i][j] is '0' or '1'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximal-square/
// discuss: https://leetcode.com/problems/maximal-square/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let row_count : usize = matrix.len();
        let col_count : usize = matrix[0].len();
        let mut heights : Vec<usize> = vec![0usize;col_count];
        let mut last_shorter_positions : Vec<i32> = vec![0i32;col_count];
        let mut next_shorter_positions : Vec<i32> = vec![0i32;col_count];

        let mut max_dim : usize = 0;
        for i in 0..row_count {
            for j in 0..col_count {
                if matrix[i][j] == '0' {
                    heights[j] = 0;
                } else {
                    heights[j] += 1;
                }
            }


            let mut index_min_stack : Vec<usize> = vec![];
            for j in 0..col_count {
                let mut last_shorter_pos : i32 = -1;
                while let Some(&last_pos) = index_min_stack.last() {
                    if heights[last_pos] < heights[j] {
                        last_shorter_pos = last_pos as i32;
                        break;
                    } else {
                        index_min_stack.pop();
                    }
                }
                index_min_stack.push(j);
                last_shorter_positions[j] = last_shorter_pos;
            }

            let mut index_min_stack : Vec<usize> = vec![];
            for j in (0..col_count).rev() {
                let mut next_shorter_pos : i32 = col_count as i32;
                while let Some(&next_pos) = index_min_stack.last() {
                    if heights[next_pos] < heights[j] {
                        next_shorter_pos = next_pos as i32;
                        break;
                    } else {
                        index_min_stack.pop();
                    }
                }
                index_min_stack.push(j);
                next_shorter_positions[j] = next_shorter_pos;
            }
            
            for j in 0..col_count {
                // println!("i={}, j={}, last_shorter_pos={}, next_shorter_pos={}, height={}", i, j, last_shorter_positions[j], next_shorter_positions[j], heights[j]);

                let width : usize = (next_shorter_positions[j] - last_shorter_positions[j] - 1) as usize;
                let this_dim : usize = std::cmp::min(width, heights[j]);
                max_dim = std::cmp::max(max_dim, this_dim);
            }
        }
        (max_dim * max_dim) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_221() {
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0'],
            ]),
            4
        );

        assert_eq!(
            Solution::maximal_square(vec![
                vec!['0','0','0','1'],
                vec!['1','1','0','1'],
                vec!['1','1','1','1'],
                vec!['0','1','1','1'],
                vec!['0','1','1','1']
            ]),
            9
        )
    }
}
