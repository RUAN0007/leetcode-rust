/**
 * [334] Increasing Triplet Subsequence
 *
 * Given an integer array nums, return true if there exists a triple of indices (i, j, k) such that i < j < k and nums[i] < nums[j] < nums[k]. If no such indices exists, return false.
 *  
 * Example 1:
 * 
 * Input: nums = [1,2,3,4,5]
 * Output: true
 * Explanation: Any triplet where i < j < k is valid.
 * 
 * Example 2:
 * 
 * Input: nums = [5,4,3,2,1]
 * Output: false
 * Explanation: No triplet exists.
 * 
 * Example 3:
 * 
 * Input: nums = [2,1,5,0,4,6]
 * Output: true
 * Explanation: The triplet (3, 4, 5) is valid because nums[3] == 0 < nums[4] == 4 < nums[5] == 6.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 5 * 10^5
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 
 *  
 * Follow up: Could you implement a solution that runs in O(n) time complexity and O(1) space complexity?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/increasing-triplet-subsequence/
// discuss: https://leetcode.com/problems/increasing-triplet-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first : i64 = 1 <<33;
        let mut sec : i64 = 1 <<33;
        for &num in nums.iter() {
            let num = num as i64;
            // println!("num={},first={},sec={}", num, first, sec);
            if num <= first {
                first = num;
            } else if num <= sec {
                sec = num;
            } else {
                return true;
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_334() {
        // assert!(Solution::increasing_triplet(vec![1,2,3,4,5]));
        // assert!(!Solution::increasing_triplet(vec![5,4,3,2,1]));
        assert!(!Solution::increasing_triplet(vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]));
    }
}
