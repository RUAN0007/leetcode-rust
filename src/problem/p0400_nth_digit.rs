/**
 * [400] Nth Digit
 *
 * Given an integer n, return the n^th digit of the infinite integer sequence [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...].
 *  
 * Example 1:
 * 
 * Input: n = 3
 * Output: 3
 * 
 * Example 2:
 * 
 * Input: n = 11
 * Output: 0
 * Explanation: The 11^th digit of the sequence 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ... is a 0, which is part of the number 10.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/nth-digit/
// discuss: https://leetcode.com/problems/nth-digit/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut n : usize = n as usize - 1;
        let mut digit_count : usize = 1;
        let mut count : usize = 9;
        let mut digit_base = 1;
        while n >= digit_count * count {
            n -= digit_count * count;
            digit_count += 1;
            digit_base *=10;
            count *= 10;
        }
        let mut num : usize = (digit_base + n / digit_count);
        (num.to_string().chars().nth(n%digit_count).unwrap() as u8 - '0' as u8) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_400() {
        // assert_eq!(Solution::find_nth_digit(3), 3);
        // assert_eq!(Solution::find_nth_digit(11), 0);
        assert_eq!(Solution::find_nth_digit(100), 5);
    }
}
