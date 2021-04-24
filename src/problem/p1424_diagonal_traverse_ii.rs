/**
 * [1424] Diagonal Traverse II
 *
 * Given a list of lists of integers, nums, return all elements of nums in diagonal order as shown in the below images.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/08/sample_1_1784.png" style="width: 158px; height: 143px;" />
 * 
 * Input: nums = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: [1,4,2,7,5,3,8,6,9]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/08/sample_2_1784.png" style="width: 230px; height: 177px;" />
 * 
 * Input: nums = [[1,2,3,4,5],[6,7],[8],[9,10,11],[12,13,14,15,16]]
 * Output: [1,6,2,8,7,3,9,4,12,10,5,13,11,14,15,16]
 * 
 * Example 3:
 * 
 * Input: nums = [[1,2,3],[4],[5,6,7],[8],[9,10,11]]
 * Output: [1,4,2,5,3,8,6,9,7,10,11]
 * 
 * Example 4:
 * 
 * Input: nums = [[1,2,3,4,5,6]]
 * Output: [1,2,3,4,5,6]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i].length <= 10^5
 * 	1 <= nums[i][j] <= 10^9
 * 	There at most 10^5 elements in nums.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/diagonal-traverse-ii/
// discuss: https://leetcode.com/problems/diagonal-traverse-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut start_positions : Vec<(i32,i32)> = vec![];
        let row_count : usize = nums.len();
        let col_count : usize = nums[0].len();
        for i in 0..row_count {
            start_positions.push((i as i32, 0 as i32));
        }
        for j in 1..col_count {
            start_positions.push((row_count as i32 - 1, j as i32));
        }

        let mut result : Vec<i32> = vec![];
        for start_pos in start_positions.iter() {
            let mut i = start_pos.0;
            let mut j = start_pos.1;
            while 0 <= i && i < row_count as i32 && 0 <=j && j < col_count as i32 {
                result.push(nums[i as usize][j as usize]);
                i-=1;
                j+=1;
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
    fn test_1424() {
        assert_eq!(Solution::find_diagonal_order(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]), vec![1,4,2,7,5,3,8,6,9]);
    }
}
