/**
 * [190] Reverse Bits
 *
 * Reverse bits of a given 32 bits unsigned integer.
 * Note:
 * 
 * 	Note that in some languages such as Java, there is no unsigned integer type. In this case, both input and output will be given as a signed integer type. They should not affect your implementation, as the integer's internal binary representation is the same, whether it is signed or unsigned.
 * 	In Java, the compiler represents the signed integers using <a href="https://en.wikipedia.org/wiki/Two%27s_complement" target="_blank">2's complement notation</a>. Therefore, in Example 2 above, the input represents the signed integer -3 and the output represents the signed integer -1073741825.
 * 
 * Follow up:
 * If this function is called many times, how would you optimize it?
 *  
 * Example 1:
 * 
 * Input: n = 00000010100101000001111010011100
 * Output:    964176192 (00111001011110000010100101000000)
 * Explanation: The input binary string 00000010100101000001111010011100 represents the unsigned integer 43261596, so return 964176192 which its binary representation is 00111001011110000010100101000000.
 * 
 * Example 2:
 * 
 * Input: n = 11111111111111111111111111111101
 * Output:   3221225471 (10111111111111111111111111111111)
 * Explanation: The input binary string 11111111111111111111111111111101 represents the unsigned integer 4294967293, so return 3221225471 which its binary representation is 10111111111111111111111111111111.
 * 
 *  
 * Constraints:
 * 
 * 	The input must be a binary string of length 32
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-bits/
// discuss: https://leetcode.com/problems/reverse-bits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut result = 0;
        let mut count = 0usize;
        while x != 0 {
            let last_digit = x & 1;
            result = (result << 1) + last_digit;
            x = x >> 1;
            count+=1;
            // println!("x={}, last_digit={}, count={}, result={:#032b}", x, last_digit, count, result);
        }
        result << (32 - count)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_190() {
        assert_eq!(Solution::reverse_bits(43261596), 964176192);
    }
}
