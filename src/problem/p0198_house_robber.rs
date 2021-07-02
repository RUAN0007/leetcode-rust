/**
 * [198] House Robber
 *
 * You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.
 * Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.
 *  
 * Example 1:
 * 
 * Input: nums = [1,2,3,1]
 * Output: 4
 * Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
 * Total amount you can rob = 1 + 3 = 4.
 * 
 * Example 2:
 * 
 * Input: nums = [2,7,9,3,1]
 * Output: 12
 * Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
 * Total amount you can rob = 2 + 9 + 1 = 12.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 100
 * 	0 <= nums[i] <= 400
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/house-robber/
// discuss: https://leetcode.com/problems/house-robber/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut prev_robbed_amount : i32 = 0;
        let mut prev_unrobbed_amount : i32 = 0;
        for &num in nums.iter() {
            let last_prev_robbed_amount = prev_robbed_amount;
            let last_prev_unrobbed_amount = prev_unrobbed_amount;

            prev_robbed_amount = last_prev_unrobbed_amount + num;
            prev_unrobbed_amount = std::cmp::max(last_prev_unrobbed_amount, last_prev_robbed_amount);
        }
        std::cmp::max(prev_robbed_amount, prev_unrobbed_amount)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_198() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(Solution::rob(vec![2, 7, 9, 10, 1]), 17);
        assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 4);
    }
}
