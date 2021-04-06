/**
 * [201] Bitwise AND of Numbers Range
 *
 * Given two integers left and right that represent the range [left, right], return the bitwise AND of all numbers in this range, inclusive.
 *  
 * Example 1:
 * 
 * Input: left = 5, right = 7
 * Output: 4
 * 
 * Example 2:
 * 
 * Input: left = 0, right = 0
 * Output: 0
 * 
 * Example 3:
 * 
 * Input: left = 1, right = 2147483647
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	0 <= left <= right <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/bitwise-and-of-numbers-range/
// discuss: https://leetcode.com/problems/bitwise-and-of-numbers-range/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
        let mut result = 0;
        if left != 0 {
            let mut bit = 0usize;
            while left != right {
                // println!("left={}, right={}, result={}, bit={}", left, right, result, bit);
                // result |= 1 << bit;
                bit += 1;
                left >>= 1;
                right >>= 1;
            }
            // println!("left={}, right={}, result={}, bit={}", left, right, result, bit);
            left * (1 << bit)
        } else {
            0
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_201() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
    }
}
