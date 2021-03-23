/**
 * [209] Minimum Size Subarray Sum
 *
 * Given an array of positive integers nums and a positive integer target, return the minimal length of a contiguous subarray [numsl, numsl+1, ..., numsr-1, numsr] of which the sum is greater than or equal to target. If there is no such subarray, return 0 instead.
 *  
 * Example 1:
 * 
 * Input: target = 7, nums = [2,3,1,2,4,3]
 * Output: 2
 * Explanation: The subarray [4,3] has the minimal length under the problem constraint.
 * 
 * Example 2:
 * 
 * Input: target = 4, nums = [1,4,4]
 * Output: 1
 * 
 * Example 3:
 * 
 * Input: target = 11, nums = [1,1,1,1,1,1,1,1]
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	1 <= target <= 10^9
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^5
 * 
 *  
 * Follow up: If you have figured out the O(n) solution, try coding another solution of which the time complexity is O(n log(n)).
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-size-subarray-sum/
// discuss: https://leetcode.com/problems/minimum-size-subarray-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, 0);
        let mut sum = nums[0];
        let mut min_length = nums.len();
        let mut found = false;
        loop {
            println!("left = {}, right = {}, sum = {}, target = {}", left, right, sum, target);
            if sum < target {
                right += 1;
                if right == nums.len() {
                    break;
                }
                sum += nums[right];
            }  else {
                if left == right {
                    return 1;
                } else if right - left + 1 <= min_length {
                    min_length = right - left + 1;
                    found = true;
                }
                sum -= nums[left];
                left += 1;
            }
        }
        if found {
            return min_length as i32;
        } else {
            return 0;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_209() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    }
}
