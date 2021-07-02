/**
 * [376] Wiggle Subsequence
 *
 * A wiggle sequence is a sequence where the differences between successive numbers strictly alternate between positive and negative. The first difference (if one exists) may be either positive or negative. A sequence with one element and a sequence with two non-equal elements are trivially wiggle sequences.
 * 
 * 	For example, [1, 7, 4, 9, 2, 5] is a wiggle sequence because the differences (6, -3, 5, -7, 3) alternate between positive and negative.
 * 	In contrast, [1, 4, 7, 2, 5] and [1, 7, 4, 5, 5] are not wiggle sequences. The first is not because its first two differences are positive, and the second is not because its last difference is zero.
 * 
 * A subsequence is obtained by deleting some elements (possibly zero) from the original sequence, leaving the remaining elements in their original order.
 * Given an integer array nums, return the length of the longest wiggle subsequence of nums.
 *  
 * Example 1:
 * 
 * Input: nums = [1,7,4,9,2,5]
 * Output: 6
 * Explanation: The entire sequence is a wiggle sequence with differences (6, -3, 5, -7, 3).
 * 
 * Example 2:
 * 
 * Input: nums = [1,17,5,10,13,15,10,5,16,8]
 * Output: 7
 * Explanation: There are several subsequences that achieve this length.
 * One is [1, 17, 10, 13, 10, 16, 8] with differences (16, -7, 3, -3, 6, -8).
 * 
 * Example 3:
 * 
 * Input: nums = [1,2,3,4,5,6,7,8,9]
 * Output: 2
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 1000
 * 	0 <= nums[i] <= 1000
 * 
 *  
 * Follow up: Could you solve this in O(n) time?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/wiggle-subsequence/
// discuss: https://leetcode.com/problems/wiggle-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut low_count : i32 = 1;
        let mut high_count : i32 = 1;
        let mut last_low : i32 = nums[0];
        let mut last_high : i32 = nums[0];

        for i in 1..nums.len() {
            let prev_last_low = last_low;
            let prev_last_high = last_high;
            let prev_low_count : i32 = low_count;
            let prev_high_count : i32 = high_count;

            if nums[i] > prev_last_low {
                last_high = nums[i];
                high_count = prev_low_count + 1;
            } else {
                last_low = nums[i]; // make it easier to wiggle in later iterations.
            }

            if nums[i] < prev_last_high {
                last_low = nums[i];
                low_count = prev_high_count + 1;
            } else {
                last_high = nums[i]; // make it easier to wiggle in later iteration
            }
        }
        std::cmp::max(low_count, high_count)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_376() {
        assert_eq!(Solution::wiggle_max_length(vec![1,7,4,9,2,5]), 6);
        assert_eq!(Solution::wiggle_max_length(vec![1,17,5,10,13,15,10,5,16,8]), 7);
        assert_eq!(Solution::wiggle_max_length(vec![1,2,3,4,5,6,7,8,9]), 2);
    }
}
