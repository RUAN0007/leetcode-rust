/**
 * [1] Two Sum
 *
 * Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 * You can return the answer in any order.
 *  
 * Example 1:
 * 
 * Input: nums = [2,7,11,15], target = 9
 * Output: [0,1]
 * Output: Because nums[0] + nums[1] == 9, we return [0, 1].
 * 
 * Example 2:
 * 
 * Input: nums = [3,2,4], target = 6
 * Output: [1,2]
 * 
 * Example 3:
 * 
 * Input: nums = [3,3], target = 6
 * Output: [0,1]
 * 
 *  
 * Constraints:
 * 
 * 	2 <= nums.length <= 10^3
 * 	-10^9 <= nums[i] <= 10^9
 * 	-10^9 <= target <= 10^9
 * 	Only one valid answer exists.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/two-sum/
// discuss: https://leetcode.com/problems/two-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num2idx = HashMap::new();
        for (idx, num) in nums.iter().enumerate() {
            let diff = target - num;
            if let Some(&diff_idx) = num2idx.get(&diff) {
                return vec![diff_idx as i32, idx as i32];
            } else {
                num2idx.insert(num, idx);
            }
        }

        // reverse.insert()
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
