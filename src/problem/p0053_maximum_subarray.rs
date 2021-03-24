/**
 * [53] Maximum Subarray
 *
 * Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
 *  
 * Example 1:
 * 
 * Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
 * Output: 6
 * Explanation: [4,-1,2,1] has the largest sum = 6.
 * 
 * Example 2:
 * 
 * Input: nums = [1]
 * Output: 1
 * 
 * Example 3:
 * 
 * Input: nums = [5,4,-1,7,8]
 * Output: 23
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 3 * 10^4
 * 	-10^5 <= nums[i] <= 10^5
 * 
 *  
 * Follow up: If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-subarray/
// discuss: https://leetcode.com/problems/maximum-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = -100000;
        let mut last_sum = -1; // the sum of max array ending at i-1 element, in each iteration. 
        for num in nums {
            let my_sum : i32; // the sum of max array ending at this element. 
            if last_sum < 0 {
                // It is better to give up with the previous array
                my_sum = num;
            } else {
                // It is better to concat with the previous array
                my_sum = last_sum + num;
            }
            max_sum = std::cmp::max(max_sum, my_sum);
            last_sum = my_sum;
        }
        max_sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_53() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![-8]), -8);
        assert_eq!(Solution::max_sub_array(vec![-8, -2]), -2);
        assert_eq!(Solution::max_sub_array(vec![5,4,-1,7,8]), 23);
    }
}
