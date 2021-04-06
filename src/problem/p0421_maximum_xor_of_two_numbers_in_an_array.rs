
/**
 * [421] Maximum XOR of Two Numbers in an Array
 *
 * Given an integer array nums, return the maximum result of nums[i] XOR nums[j], where 0 &le; i &le; j < n.
 * Follow up: Could you do this in O(n) runtime?
 *  
 * Example 1:
 * 
 * Input: nums = [3,10,5,25,2,8]
 * Output: 28
 * Explanation: The maximum result is 5 XOR 25 = 28.
 * Example 2:
 * 
 * Input: nums = [0]
 * Output: 0
 * 
 * Example 3:
 * 
 * Input: nums = [2,4]
 * Output: 6
 * 
 * Example 4:
 * 
 * Input: nums = [8,10,2]
 * Output: 10
 * 
 * Example 5:
 * 
 * Input: nums = [14,70,53,83,49,91,36,80,92,51,66,70]
 * Output: 127
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 2 * 10^4
 * 	0 <= nums[i] <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/
// discuss: https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashSet;
impl Solution {
    // For each iteration at i, we assume to the left parts from i-th digit of the final result, as max_result.
    // We try to examine whether [max_result][10000.0] can be attainable. 
    // If so, max_result = [max_result][10000.0], else [max_result][o0000.0] for the next iteration. 

    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut max_result = 0;
        let mut mask = 0;
        for i in (0..32usize).rev() {
            mask = mask | (1 << i); // mask to get leftmost i digits
            let mut exists = HashSet::new();
            let guess = max_result | (1 << i);
            // println!("i:{}, mask: {}, max_result={}, guess={}, exists={:?}", i, mask, max_result, guess, exists);
            for &num in &nums {
                // println!("\texists: num={}, num&mask={}", num, num&mask);
                exists.insert(num & mask);
            }

            for &num in &nums {
                // println!("\tcheck: num={}, num^mask={}", num, num ^ guess);
                if exists.contains(&(num & mask ^ guess)) {
                    max_result = guess;
                    break;
                }
            }
            // println!("");
        }
        max_result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_421() {
        assert_eq!(Solution::find_maximum_xor(vec![3,10,5,25,2,8]), 28)
    }
}
