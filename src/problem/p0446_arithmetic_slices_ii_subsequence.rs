/**
 * [446] Arithmetic Slices II - Subsequence
 *
 * Given an integer array nums, return the number of all the arithmetic subsequences of nums.
 * A sequence of numbers is called arithmetic if it consists of at least three elements and if the difference between any two consecutive elements is the same.
 * 
 * 	For example, [1, 3, 5, 7, 9], [7, 7, 7, 7], and [3, -1, -5, -9] are arithmetic sequences.
 * 	For example, [1, 1, 2, 5, 7] is not an arithmetic sequence.
 * 
 * A subsequence of an array is a sequence that can be formed by removing some elements (possibly none) of the array.
 * 
 * 	For example, [2,5,10] is a subsequence of [1,2,1,<u>2</u>,4,1,<u>5</u>,<u>10</u>].
 * 
 * The answer is guaranteed to fit in 32-bit integer.
 *  
 * Example 1:
 * 
 * Input: nums = [2,4,6,8,10]
 * Output: 7
 * Explanation: All arithmetic subsequence slices are:
 * [2,4,6]
 * [4,6,8]
 * [6,8,10]
 * [2,4,6,8]
 * [4,6,8,10]
 * [2,4,6,8,10]
 * [2,6,10]
 * 
 * Example 2:
 * 
 * Input: nums = [7,7,7,7,7]
 * Output: 16
 * Explanation: Any subsequence of this array is arithmetic.
 * 
 *  
 * Constraints:
 * 
 * 	1  <= nums.length <= 1000
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/arithmetic-slices-ii-subsequence/
// discuss: https://leetcode.com/problems/arithmetic-slices-ii-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n : usize = nums.len();
        let nums : Vec<i64> = nums.into_iter().map(|x|{x as i64}).collect();
        // counts[i][d] counts the number of arithmetic subsequences ending at nums[i] with diff d. 
        //    the sequence can be of length 2.  
        let mut counts : Vec<HashMap<i64, usize>> = vec![HashMap::new();n];
        let mut result : usize = 0;
        for i in 1..n {
            for j in 0..i {
                let diff : i64 = nums[i] - nums[j];
                let mut prev : usize = 0;
                if let Some(&prev_count) = counts[j].get(&diff) {
                    prev = prev_count;
                }

                // +1 to account for two-value subsequence with [num[j], num[i]]
                *counts[i].entry(diff).or_insert(0usize) += prev + 1;
                result += prev;
            }
        }
        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_446() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![2,4,6,8,10]), 7);
        assert_eq!(Solution::number_of_arithmetic_slices(vec![7,7,7,7,7]), 16);
    }
}
