/**
 * [287] Find the Duplicate Number
 *
 * Given an array of integers nums containing n + 1 integers where each integer is in the range [1, n] inclusive.
 * There is only one repeated number in nums, return this repeated number.
 *  
 * Example 1:
 * Input: nums = [1,3,4,2,2]
 * Output: 2
 * Example 2:
 * Input: nums = [3,1,3,4,2]
 * Output: 3
 * Example 3:
 * Input: nums = [1,1]
 * Output: 1
 * Example 4:
 * Input: nums = [1,1,2]
 * Output: 1
 *  
 * Constraints:
 * 
 * 	2 <= n <= 3 * 10^4
 * 	nums.length == n + 1
 * 	1 <= nums[i] <= n
 * 	All the integers in nums appear only once except for precisely one integer which appears two or more times.
 * 
 *  
 * Follow up:
 * 
 * 	How can we prove that at least one duplicate number must exist in nums?
 * 	Can you solve the problem without modifying the array nums?
 * 	Can you solve the problem using only constant, O(1) extra space?
 * 	Can you solve the problem with runtime complexity less than O(n^2)?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-duplicate-number/
// discuss: https://leetcode.com/problems/find-the-duplicate-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
         nums.sort();
         for i in 1..nums.len() {
            if nums[i-1] == nums[i] {return nums[i-1];}
         }
         panic!("Can not reach here");
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_287() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
        assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 5]), 5);
        assert_eq!(Solution::find_duplicate(vec![5, 1, 2, 3, 4, 5]), 5);
    }
}
