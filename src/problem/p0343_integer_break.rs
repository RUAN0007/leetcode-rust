/**
 * [343] Integer Break
 *
 * Given an integer n, break it into the sum of k positive integers, where k >= 2, and maximize the product of those integers.
 * Return the maximum product you can get.
 *  
 * Example 1:
 * 
 * Input: n = 2
 * Output: 1
 * Explanation: 2 = 1 + 1, 1 &times; 1 = 1.
 * 
 * Example 2:
 * 
 * Input: n = 10
 * Output: 36
 * Explanation: 10 = 3 + 3 + 4, 3 &times; 3 &times; 4 = 36.
 * 
 *  
 * Constraints:
 * 
 * 	2 <= n <= 58
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/integer-break/
// discuss: https://leetcode.com/problems/integer-break/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let mut result = vec![1;(n+1) as usize];
        for i in 2..=(n as usize) {
            for j in 1..=((i-1) as usize) {
                // println!("i={}, j={}, j* result[i-j]={}", i, j, (j as i32) * result[i-j]);
                result[i] = std::cmp::max(result[i], (j as i32) * result[i-j]);
                result[i] = std::cmp::max(result[i], (j as i32) * (i-j) as i32);
            }
                // println!("i={}, result[i]={}", i, result[i]);
        }
        result[n as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_343() {
        // assert_eq!(Solution::integer_break(2), 1);
        assert_eq!(Solution::integer_break(10), 36);
    }
}
