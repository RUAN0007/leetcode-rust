/**
 * [120] Triangle
 *
 * Given a triangle array, return the minimum path sum from top to bottom.
 * For each step, you may move to an adjacent number of the row below. More formally, if you are on index i on the current row, you may move to either index i or index i + 1 on the next row.
 *  
 * Example 1:
 * 
 * Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
 * Output: 11
 * Explanation: The triangle looks like:
 *    <u>2</u>
 *   <u>3</u> 4
 *  6 <u>5</u> 7
 * 4 <u>1</u> 8 3
 * The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11 (underlined above).
 * 
 * Example 2:
 * 
 * Input: triangle = [[-10]]
 * Output: -10
 * 
 *  
 * Constraints:
 * 
 * 	1 <= triangle.length <= 200
 * 	triangle[0].length == 1
 * 	triangle[i].length == triangle[i - 1].length + 1
 * 	-10^4 <= triangle[i][j] <= 10^4
 * 
 *  
 * Follow up: Could you do this using only O(n) extra space, where n is the total number of rows in the triangle?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/triangle/
// discuss: https://leetcode.com/problems/triangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut sum = triangle.clone();
        let level_count = triangle.len();
        for i in 1..level_count {
            // let prev_level_sum = &sum[i-1];
            let level = &triangle[i];
            for (j, &num) in level.iter().enumerate() {
                let mut min_prev_level = 10000;
                if 0 < j {
                    min_prev_level = std::cmp::min(min_prev_level, sum[i-1][j-1]);
                }

                if sum[i-1].get(j).is_some() {
                    min_prev_level = std::cmp::min(min_prev_level, sum[i-1][j]);
                }
                sum[i][j] = min_prev_level + num;
            }
        }
        // println!("sum: {:?}", sum);
        let mut lowest_min = 10000;
        for &num in sum.last().unwrap().iter() {
            lowest_min = std::cmp::min(lowest_min, num);
        }
        lowest_min
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_120() {
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        )
    }
}
