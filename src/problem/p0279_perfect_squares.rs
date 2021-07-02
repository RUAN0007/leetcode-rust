/**
 * [279] Perfect Squares
 *
 * Given an integer n, return the least number of perfect square numbers that sum to n.
 * A perfect square is an integer that is the square of an integer; in other words, it is the product of some integer with itself. For example, 1, 4, 9, and 16 are perfect squares while 3 and 11 are not.
 *  
 * Example 1:
 * 
 * Input: n = 12
 * Output: 3
 * Explanation: 12 = 4 + 4 + 4.
 * 
 * Example 2:
 * 
 * Input: n = 13
 * Output: 2
 * Explanation: 13 = 4 + 9.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/perfect-squares/
// discuss: https://leetcode.com/problems/perfect-squares/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n : usize = n as usize;
        // dp[i] denotes for the min number of square which sum to i.
        let mut dp : Vec<i32> = vec![0;n+1];
        for i in 1..=n {
            let mut j : usize = 1;
            let mut prev_min : i32 = n as i32;
            while j*j <= i {
                prev_min = std::cmp::min(prev_min, dp[i-j*j]);
                j+=1;
            }
            dp[i] = prev_min + 1;
        }

        dp[n]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_279() {
        assert_eq!(Solution::num_squares(13), 2);
        assert_eq!(Solution::num_squares(12), 3);
    }
}
