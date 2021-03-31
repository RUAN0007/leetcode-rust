/**
 * [29] Divide Two Integers
 *
 * Given two integers dividend and divisor, divide two integers without using multiplication, division, and mod operator.
 * Return the quotient after dividing dividend by divisor.
 * The integer division should truncate toward zero, which means losing its fractional part. For example, truncate(8.345) = 8 and truncate(-2.7335) = -2.
 * Note: Assume we are dealing with an environment that could only store integers within the 32-bit signed integer range: [-2^31, 2^31 - 1]. For this problem, assume that your function returns 2^31 - 1 when the division result overflows.
 *  
 * Example 1:
 * 
 * Input: dividend = 10, divisor = 3
 * Output: 3
 * Explanation: 10/3 = truncate(3.33333..) = 3.
 * 
 * Example 2:
 * 
 * Input: dividend = 7, divisor = -3
 * Output: -2
 * Explanation: 7/-3 = truncate(-2.33333..) = -2.
 * 
 * Example 3:
 * 
 * Input: dividend = 0, divisor = 1
 * Output: 0
 * 
 * Example 4:
 * 
 * Input: dividend = 1, divisor = 1
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	-2^31 <= dividend, divisor <= 2^31 - 1
 * 	divisor != 0
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/divide-two-integers/
// discuss: https://leetcode.com/problems/divide-two-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn divide(mut dividend: i64, mut divisor: i64) -> i32 {
        let mut sign_flipped = false;
        let mut abs_result = 0i64;
        if 0 < dividend && 0 < divisor {
            sign_flipped = false;
        } else if dividend < 0 && 0 < divisor {
            dividend = -dividend;
            sign_flipped = true;
        } else if 0 < dividend && divisor < 0 {
            sign_flipped = true;
            divisor = -divisor;
        } else if dividend < 0 && divisor < 0 {
            sign_flipped = false;
            divisor = -divisor;
            dividend = -dividend;
        } else if dividend == 0 {
            return 0;
        }


        let mut remains = dividend;
        loop {
            let mut cur_shift_count = -1;
            let mut cur_subtractor = divisor;
            let mut last_subtractor = divisor; // the init value is meaningless

            while cur_subtractor <= remains {
                cur_shift_count += 1;
                last_subtractor = cur_subtractor;
                cur_subtractor = cur_subtractor << 1; // *2
            }
            if cur_shift_count == -1 {
                break
            } else {
                abs_result += 1 << cur_shift_count;
                remains -= last_subtractor;
            }
        }


        if sign_flipped {
            -(abs_result as i32)
        } else {
            abs_result as i32
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_29() {
        // assert_eq!(Solution::divide(10, 3), 3);
        // assert_eq!(Solution::divide(7, -3), -2);
        assert_eq!(Solution::divide(2147483647, 2), 1073741823);

    }
}
