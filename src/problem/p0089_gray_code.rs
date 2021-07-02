/**
 * [89] Gray Code
 *
 * An n-bit gray code sequence is a sequence of 2^n integers where:
 * 
 * 	Every integer is in the inclusive range [0, 2^n - 1],
 * 	The first integer is 0,
 * 	An integer appears no more than once in the sequence,
 * 	The binary representation of every pair of adjacent integers differs by exactly one bit, and
 * 	The binary representation of the first and last integers differs by exactly one bit.
 * 
 * Given an integer n, return any valid n-bit gray code sequence.
 *  
 * Example 1:
 * 
 * Input: n = 2
 * Output: [0,1,3,2]
 * Explanation:
 * The binary representation of [0,1,3,2] is [00,01,11,10].
 * - 0<u>0</u> and 0<u>1</u> differ by one bit
 * - <u>0</u>1 and <u>1</u>1 differ by one bit
 * - 1<u>1</u> and 1<u>0</u> differ by one bit
 * - <u>1</u>0 and <u>0</u>0 differ by one bit
 * [0,2,3,1] is also a valid gray code sequence, whose binary representation is [00,10,11,01].
 * - <u>0</u>0 and <u>1</u>0 differ by one bit
 * - 1<u>0</u> and 1<u>1</u> differ by one bit
 * - <u>1</u>1 and <u>0</u>1 differ by one bit
 * - 0<u>1</u> and 0<u>0</u> differ by one bit
 * 
 * Example 2:
 * 
 * Input: n = 1
 * Output: [0,1]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 16
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/gray-code/
// discuss: https://leetcode.com/problems/gray-code/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let n : usize = n as usize;
        let count : usize = 1 << n;
        let mut result : Vec<i32> = vec![];
        for i in 0..count {
            let mut num : i32 = 0;
            for bit_pos in 0..n {
                let period : usize = 1 << (bit_pos + 2);
                if (1 << bit_pos) <= i % period && i % period < period - (1 << bit_pos) {
                    num |= 1 << bit_pos;
                }
            }

            result.push(num);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_89() {
        assert_eq!(Solution::gray_code(2), vec![0, 1, 3, 2]);
        assert_eq!(Solution::gray_code(1), vec![0, 1]);
        assert_eq!(Solution::gray_code(0), vec![0]);
        assert_eq!(Solution::gray_code(3), vec![0, 1, 3, 2, 6, 7, 5, 4]);
    }
}
