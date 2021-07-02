/**
 * [55] Jump Game
 *
 * Given an array of non-negative integers nums, you are initially positioned at the first index of the array.
 * Each element in the array represents your maximum jump length at that position.
 * Determine if you are able to reach the last index.
 *  
 * Example 1:
 * 
 * Input: nums = [2,3,1,1,4]
 * Output: true
 * Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
 * 
 * Example 2:
 * 
 * Input: nums = [3,2,1,0,4]
 * Output: false
 * Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^4
 * 	0 <= nums[i] <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/jump-game/
// discuss: https://leetcode.com/problems/jump-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut cur_pos : usize = 0;
        let mut max_pos : usize = cur_pos + nums[cur_pos] as usize;
        while max_pos < nums.len() - 1 {
            // println!("cur_pos={}, max_pos={}", cur_pos, max_pos);
            let mut next_max_pos : usize = 0;
            for pos in (cur_pos+1)..=max_pos {
                next_max_pos = std::cmp::max(next_max_pos, pos + nums[pos] as usize);
            }
            if max_pos == next_max_pos {
                return false;
            } else {
                cur_pos = max_pos;
                max_pos = next_max_pos;
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_55() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 0, 0, 0, 4]), false);
        assert_eq!(Solution::can_jump(vec![8, 3, 1, 1, 0, 0, 0, 4]), true);
        assert_eq!(Solution::can_jump(vec![0]), true);
        assert_eq!(Solution::can_jump(vec![1, 1, 2, 2, 0, 1, 1]), true);
        assert_eq!(
            Solution::can_jump(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0]),
            true
        );
    }
}
