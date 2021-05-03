/**
 * [41] First Missing Positive
 *
 * Given an unsorted integer array nums, find the smallest missing positive integer.
 *  
 * Example 1:
 * Input: nums = [1,2,0]
 * Output: 3
 * Example 2:
 * Input: nums = [3,4,-1,1]
 * Output: 2
 * Example 3:
 * Input: nums = [7,8,9,11,12]
 * Output: 1
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 300
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 
 *  
 * Follow up: Could you implement an algorithm that runs in O(n) time and uses constant extra space?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/first-missing-positive/
// discuss: https://leetcode.com/problems/first-missing-positive/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut bit_set : Vec<u64> = vec![0u64;5];

        for &num in nums.iter() {
            if num <= 0 {continue;}
            let num = num as usize;
            let set_idx : usize = num / 64;
            if set_idx >= 5 {continue;}
            let bit_idx : usize = num % 64;
            bit_set[set_idx] |= 1 << bit_idx;
        }

        for bit_pos in 1..=301 {
            let set_idx : usize = bit_pos / 64;
            let bit_idx : usize = bit_pos % 64;
            if bit_set[set_idx] & (1 << bit_idx) == 0 {
                return bit_pos as i32;
            }

        }
        panic!("Shall not reach here...");
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_41() {
        assert_eq!(Solution::first_missing_positive(vec![2, 2]), 1);
        assert_eq!(
            Solution::first_missing_positive(vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2]),
            1
        );
        assert_eq!(
            Solution::first_missing_positive(vec![2, 2, 2, 2, 2, 2, 2]),
            1
        );
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![2, 1, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
        assert_eq!(
            Solution::first_missing_positive(vec![7, 8, 1, 2, 3, 3, 3, 3, 3, 3, 3, -5, -7, 1234]),
            4
        );
    }
}
