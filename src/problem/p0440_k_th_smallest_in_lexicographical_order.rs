/**
 * [440] K-th Smallest in Lexicographical Order
 *
 * Given two integers n and k, return the k^th lexicographically smallest integer in the range [1, n].
 *  
 * Example 1:
 * 
 * Input: n = 13, k = 2
 * Output: 10
 * Explanation: The lexicographical order is [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9], so the second smallest number is 10.
 * 
 * Example 2:
 * 
 * Input: n = 1, k = 1
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= k <= n <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/
// discuss: https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut k = k - 1;
        let mut cur : i32 = 1;
        while 0 < k {
            let step : i32 = Self::cal_steps(n as i64, cur as i64, cur as i64 + 1) as i32;
            if step <= k {
                k -= step;
                cur += 1;
            } else {
                k -= 1;
                cur *= 10;
            }
        }
        cur
    }

    pub fn cal_steps(n : i64, mut level_start : i64, mut level_end : i64) -> i64 {
        let mut steps : i64 = 0;
        while level_start <= n {
            steps += std::cmp::min(n + 1, level_end) - level_start;
            level_start *= 10;
            level_end *= 10;
        }
        steps
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_440() {
    }
}
