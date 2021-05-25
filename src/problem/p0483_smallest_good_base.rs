/**
 * [483] Smallest Good Base
 *
 * Given an integer n represented as a string, return the smallest good base of n.
 * We call k >= 2 a good base of n, if all digits of n base k are 1's.
 *  
 * Example 1:
 * 
 * Input: n = "13"
 * Output: "3"
 * Explanation: 13 base 3 is 111.
 * 
 * Example 2:
 * 
 * Input: n = "4681"
 * Output: "8"
 * Explanation: 4681 base 8 is 11111.
 * 
 * Example 3:
 * 
 * Input: n = "1000000000000000000"
 * Output: "999999999999999999"
 * Explanation: 1000000000000000000 base 999999999999999999 is 11.
 * 
 *  
 * Constraints:
 * 
 * 	n is an integer in the range [3, 10^18].
 * 	n does not contain any leading zeros.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-good-base/
// discuss: https://leetcode.com/problems/smallest-good-base/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        // assume the base is k and the number of digits is m. 
        let n : u128 = n.parse::<u128>().unwrap();
        let max_m : u128 = (n as f64).log(2.0f64) as u128;
        for m in (1..=max_m).rev() {
            let k : f64 = (n as f64).powf(1f64 / (m as f64));
            let k : u128 = k.floor() as u128;
            if (k.pow(m as u32 +1)-1)/(k-1) == n && (k.pow(m as u32 +1)-1) % (k-1)==0 {
                return k.to_string();
            }
        }
        (n-1).to_string()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_483() {
        assert_eq!(Solution::smallest_good_base("13".to_owned()), "3".to_owned());
        assert_eq!(Solution::smallest_good_base("4681".to_owned()), "8".to_owned());
        assert_eq!(Solution::smallest_good_base("1000000000000000000".to_owned()), "999999999999999999".to_owned());
    }
}
