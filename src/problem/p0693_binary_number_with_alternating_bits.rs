/**
 * [693] Binary Number with Alternating Bits
 *
 * Given a positive integer, check whether it has alternating bits: namely, if two adjacent bits will always have different values.
 *  
 * Example 1:
 * 
 * Input: n = 5
 * Output: true
 * Explanation: The binary representation of 5 is: 101
 * 
 * Example 2:
 * 
 * Input: n = 7
 * Output: false
 * Explanation: The binary representation of 7 is: 111.
 * Example 3:
 * 
 * Input: n = 11
 * Output: false
 * Explanation: The binary representation of 11 is: 1011.
 * Example 4:
 * 
 * Input: n = 10
 * Output: true
 * Explanation: The binary representation of 10 is: 1010.
 * Example 5:
 * 
 * Input: n = 3
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/binary-number-with-alternating-bits/
// discuss: https://leetcode.com/problems/binary-number-with-alternating-bits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let n = n ^ n >> 1;
        (n & (n+1)) == 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_693() {
        assert!(Solution::has_alternating_bits(5));
        assert!(!Solution::has_alternating_bits(7));
        assert!(!Solution::has_alternating_bits(11));
        assert!(Solution::has_alternating_bits(10));
    }
}
