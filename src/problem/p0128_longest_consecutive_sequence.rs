/**
 * [128] Longest Consecutive Sequence
 *
 * Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.
 *  
 * Example 1:
 * 
 * Input: nums = [100,4,200,1,3,2]
 * Output: 4
 * Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
 * 
 * Example 2:
 * 
 * Input: nums = [0,3,7,2,5,8,4,6,0,1]
 * Output: 9
 * 
 *  
 * Constraints:
 * 
 * 	0 <= nums.length <= 10^4
 * 	-10^9 <= nums[i] <= 10^9
 * 
 *  
 * Follow up: Could you implement the O(n) solution?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-consecutive-sequence/
// discuss: https://leetcode.com/problems/longest-consecutive-sequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut num_set = HashSet::new();
        for &num in &nums {
            num_set.insert(num);
        }

        let mut longest = 0;
        for &num in &nums {
            if !num_set.contains(&(num-1)) {
                // num can be a potential start of consecutive nums
                let mut next = num + 1;
                let mut count = 1;
                while num_set.contains(&next) {
                    count += 1;
                    next += 1;
                }
                longest = std::cmp::max(longest, count);
            }
        }

        longest
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_128() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4)
    }
}
