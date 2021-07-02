/**
 * [166] Fraction to Recurring Decimal
 *
 * Given two integers representing the numerator and denominator of a fraction, return the fraction in string format.
 * If the fractional part is repeating, enclose the repeating part in parentheses.
 * If multiple answers are possible, return any of them.
 * It is guaranteed that the length of the answer string is less than 10^4 for all the given inputs.
 *  
 * Example 1:
 * Input: numerator = 1, denominator = 2
 * Output: "0.5"
 * Example 2:
 * Input: numerator = 2, denominator = 1
 * Output: "2"
 * Example 3:
 * Input: numerator = 2, denominator = 3
 * Output: "0.(6)"
 * Example 4:
 * Input: numerator = 4, denominator = 333
 * Output: "0.(012)"
 * Example 5:
 * Input: numerator = 1, denominator = 5
 * Output: "0.2"
 *  
 * Constraints:
 * 
 * 	-2^31 <= numerator, denominator <= 2^31 - 1
 * 	denominator != 0
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/fraction-to-recurring-decimal/
// discuss: https://leetcode.com/problems/fraction-to-recurring-decimal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let numerator : i64 = numerator as i64;
        let denominator : i64 = denominator as i64;
        let mut neg : bool = (numerator * denominator) < 0;
        let numerator = i64::abs(numerator);
        let denominator = i64::abs(denominator);

        let int_part : i64 = numerator / denominator;
        let mut frac_part : Vec<i64> = vec![];
        let mut repeated_pos : Option<usize> = None;

        let mut remainder : i64 = numerator % denominator;
        let mut cache : HashMap<i64, usize> = HashMap::new();
        while remainder != 0 {
            cache.insert(remainder, frac_part.len());
            remainder *=10;
            while remainder < denominator {
                frac_part.push(0);
                remainder *= 10;
            }
            frac_part.push(remainder / denominator);
            remainder = remainder % denominator;
            if let Some(&pos) = cache.get(&remainder) {
                repeated_pos = Some(pos);
                break;
            }
        }
        let mut result : String = "".to_owned();
        if neg {
            result.push('-');
        }
        result.push_str(&int_part.to_string());
        if frac_part.len() == 0 {
            // do nothing
        } else if let Some(repeated_pos) = repeated_pos {
            result.push('.');
            let no_repeated : String = frac_part[0..repeated_pos].iter().
                map(|&x|{(x as u8 + '0' as u8) as char}).collect();
            result.push_str(&no_repeated);
            result.push('(');
            let repeated : String = frac_part[repeated_pos..].iter()
                .map(|&x|{(x as u8 + '0' as u8) as char}).collect();
            result.push_str(&repeated);
            result.push(')');
        } else {
            result.push('.');
            let no_repeated : String = frac_part[..].iter()
                .map(|&x|{(x as u8 + '0' as u8) as char}).collect();
            result.push_str(&no_repeated);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_166() {
        // assert_eq!(Solution::fraction_to_decimal(1, 2), "0.5".to_owned());
        // assert_eq!(Solution::fraction_to_decimal(2, 1), "2".to_owned());
        // assert_eq!(Solution::fraction_to_decimal(2, 3), "0.(6)".to_owned());
        // assert_eq!(Solution::fraction_to_decimal(4, 333), "0.(012)".to_owned());
        // assert_eq!(Solution::fraction_to_decimal(1, 5), "0.2".to_owned());
        // assert_eq!(Solution::fraction_to_decimal(-50, 8), "-6.25".to_owned());
        assert_eq!(Solution::fraction_to_decimal(-1, -2147483648), "0.0000000004656612873077392578125".to_owned());
    }
}
