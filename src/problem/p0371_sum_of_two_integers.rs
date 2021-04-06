/**
 * [371] Sum of Two Integers
 *
 * Given two integers a and b, return the sum of the two integers without using the operators + and -.
 *  
 * Example 1:
 * Input: a = 1, b = 2
 * Output: 3
 * Example 2:
 * Input: a = 2, b = 3
 * Output: 5
 *  
 * Constraints:
 * 
 * 	-1000 <= a, b <= 1000
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-two-integers/
// discuss: https://leetcode.com/problems/sum-of-two-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        if a == 0 {
            b
        } else if b == 0 {
            a
        } else {
            while b != 0 {
                let carry = a & b;
                a ^= b;
                b = carry << 1;
            }
            a
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_371() {
    }
}
