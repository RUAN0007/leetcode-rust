/**
 * [260] Single Number III
 *
 * Given an integer array nums, in which exactly two elements appear only once and all the other elements appear exactly twice. Find the two elements that appear only once. You can return the answer in any order.
 * Follow up: Your algorithm should run in linear runtime complexity. Could you implement it using only constant space complexity?
 *  
 * Example 1:
 * 
 * Input: nums = [1,2,1,3,2,5]
 * Output: [3,5]
 * Explanation:  [5, 3] is also a valid answer.
 * 
 * Example 2:
 * 
 * Input: nums = [-1,0]
 * Output: [-1,0]
 * 
 * Example 3:
 * 
 * Input: nums = [0,1]
 * Output: [1,0]
 * 
 *  
 * Constraints:
 * 
 * 	2 <= nums.length <= 3 * 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	Each integer in nums will appear twice, only two integers will appear once.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/single-number-iii/
// discuss: https://leetcode.com/problems/single-number-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        // suppose two distinct numbers are a and b.
        let axorb  = nums.iter().fold(0, |acc, &x| {acc^x});
        // a, and b differ in this bit. 
        let last_set = axorb & -axorb;
        // either a or b with the last_set-th bit 0 uniquely in this category
        let num0 = nums.iter().filter(|x|{(**x & last_set)==0}).fold(0, |acc, &x|{acc^x});
        // either a or b with the last_set-th bit 1 uniquely in this category
        let num1 = nums.iter().filter(|x|{(**x & last_set)!=0}).fold(0, |acc, &x|{acc^x});

        vec![num0, num1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_260() {
        // assert_eq!(Solution::single_number(vec![1, 2, 1, 2, 3, 4]), vec![3, 4]);
        assert_eq!(Solution::single_number(vec![1, 2, 1, 3, 2, 5]), vec![3, 5]);
    }
}
