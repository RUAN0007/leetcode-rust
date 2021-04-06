/**
 * [229] Majority Element II
 *
 * Given an integer array of size n, find all elements that appear more than &lfloor; n/3 &rfloor; times.
 * Follow-up: Could you solve the problem in linear time and in O(1) space?
 *  
 * Example 1:
 * 
 * Input: nums = [3,2,3]
 * Output: [3]
 * 
 * Example 2:
 * 
 * Input: nums = [1]
 * Output: [1]
 * 
 * Example 3:
 * 
 * Input: nums = [1,2]
 * Output: [1,2]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 5 * 10^4
 * 	-10^9 <= nums[i] <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/majority-element-ii/
// discuss: https://leetcode.com/problems/majority-element-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_229() {
        assert_eq!(
            Solution::majority_element(vec![1, 1, 1, 2, 2, 2, 3, 3, 3]),
            vec![]
        );
        assert_eq!(
            Solution::majority_element(vec![1, 1, 1, 2, 2, 3, 3, 3]),
            vec![1, 3]
        );
        assert_eq!(Solution::majority_element(vec![1]), vec![1]);
        assert_eq!(Solution::majority_element(vec![5, 6, 6]), vec![6]);
        assert_eq!(Solution::majority_element(vec![1, 2, 3, 4]), vec![]);
    }
}
