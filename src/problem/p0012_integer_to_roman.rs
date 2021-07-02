/**
 * [12] Integer to Roman
 *
 * Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
 * 
 * Symbol       Value
 * I             1
 * V             5
 * X             10
 * L             50
 * C             100
 * D             500
 * M             1000
 * For example, 2 is written as II in Roman numeral, just two one's added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
 * Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
 * 
 * 	I can be placed before V (5) and X (10) to make 4 and 9. 
 * 	X can be placed before L (50) and C (100) to make 40 and 90. 
 * 	C can be placed before D (500) and M (1000) to make 400 and 900.
 * 
 * Given an integer, convert it to a roman numeral.
 *  
 * Example 1:
 * 
 * Input: num = 3
 * Output: "III"
 * 
 * Example 2:
 * 
 * Input: num = 4
 * Output: "IV"
 * 
 * Example 3:
 * 
 * Input: num = 9
 * Output: "IX"
 * 
 * Example 4:
 * 
 * Input: num = 58
 * Output: "LVIII"
 * Explanation: L = 50, V = 5, III = 3.
 * 
 * Example 5:
 * 
 * Input: num = 1994
 * Output: "MCMXCIV"
 * Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= num <= 3999
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/integer-to-roman/
// discuss: https://leetcode.com/problems/integer-to-roman/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut result : Vec<char> = vec![];
        let m_count : i32 = num / 1000i32;
        for i in 0..m_count {
            result.push('M');
        }
        num = num % 1000i32;
        if num >= 900 {
            result.push('C');
            result.push('M');
            num -= 900;
        } else if num >= 500 {
            result.push('D');
            num -= 500;
        } else if num >= 400 {
            result.push('C');
            result.push('D');
            num -= 400;
        }

        let c_count : i32 = num / 100i32;
        num = num % 100i32;
        for i in 0..c_count {
            result.push('C');
        }
        if num >= 90 {
            result.push('X');
            result.push('C');
            num -= 90;
        } else if num >= 50 {
            result.push('L');
            num -= 50;
        } else if num >= 40 {
            result.push('X');
            result.push('L');
            num -= 40;
        }

        let x_count : i32 = num / 10i32;
        num = num % 10;
        for i in 0..x_count {
            result.push('X');
        }

        if num >= 9 {
            result.push('I');
            result.push('X');
            num -= 9;
        } else if num >= 5 {
            result.push('V');
            num -= 5;
        } else if num >= 4 {
            result.push('I');
            result.push('V');
            num -= 40;
        }

        for i in 0..num {
            result.push('I');
        }
        result.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_12() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
