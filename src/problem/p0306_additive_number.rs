/**
 * [306] Additive Number
 *
 * Additive number is a string whose digits can form additive sequence.
 * A valid additive sequence should contain at least three numbers. Except for the first two numbers, each subsequent number in the sequence must be the sum of the preceding two.
 * Given a string containing only digits '0'-'9', write a function to determine if it's an additive number.
 * Note: Numbers in the additive sequence cannot have leading zeros, so sequence 1, 2, 03 or 1, 02, 3 is invalid.
 *  
 * Example 1:
 * 
 * Input: "112358"
 * Output: true
 * Explanation: The digits can form an additive sequence: 1, 1, 2, 3, 5, 8. 
 *              1 + 1 = 2, 1 + 2 = 3, 2 + 3 = 5, 3 + 5 = 8
 * 
 * Example 2:
 * 
 * Input: "199100199"
 * Output: true
 * Explanation: The additive sequence is: 1, 99, 100, 199. 
 *              1 + 99 = 100, 99 + 100 = 199
 * 
 *  
 * Constraints:
 * 
 * 	<font face="monospace">num </font>consists only of digits '0'-'9'.
 * 	1 <= num.length <= 35
 * 
 * Follow up:<br />
 * How would you handle overflow for very large input integers?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/additive-number/
// discuss: https://leetcode.com/problems/additive-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn digits2num(digits : &Vec<char>, start : usize, end : usize) -> i64 {
        let mut num : i64 = 0;
        for i in start..=end {
            num = 10 * num + (digits[i] as u8 - '0' as u8) as i64;
        }
        num
    }
    pub fn helper(digits : &Vec<char>, cur_pos : usize, tmp : &mut Vec<i64>) -> bool {
        let digit_len : usize = digits.len();
        if cur_pos ==  digit_len { 
            return tmp.len() >= 3;
        }
        let mut leading_zero : bool = false;
        if digits[cur_pos] == '0' { leading_zero = true; }

        if tmp.len() == 0 || tmp.len() == 1 {
            for end_pos in cur_pos..digit_len {
                if leading_zero && end_pos > cur_pos { break; }

                let num : i64 = Self::digits2num(&digits, cur_pos, end_pos);
                tmp.push(num);
                if Self::helper(&digits, end_pos+1, tmp) {
                    return true;
                } else {
                    tmp.pop();
                }
            }
            false
        } else {
            let target_sum : i64 = tmp[tmp.len() - 1] + tmp[tmp.len() - 2];
            let mut end_pos : usize = cur_pos;
            let mut num : i64 = (digits[end_pos] as u8 - '0' as u8) as i64;

            while !leading_zero && num < target_sum && end_pos+1 < digit_len {
                end_pos+=1;
                num = 10 * num + (digits[end_pos] as u8 - '0' as u8) as i64;
            }

            if num < target_sum {
                // out of digits;
                false
            } else if num == target_sum {
                tmp.push(num);
                let result = Self::helper(&digits, end_pos + 1, tmp);
                tmp.pop();
                result
            } else {
                false
            }
        }
    }

    pub fn is_additive_number(num: String) -> bool {
        let digits : Vec<char> = num.chars().collect();
        let mut tmp : Vec<i64> = vec![];
        Self::helper(&digits, 0, &mut tmp)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_306() {
        // assert_eq!(Solution::is_additive_number("112358".to_owned()), true);
        // assert_eq!(Solution::is_additive_number("199100199".to_owned()), true);
        // assert_eq!(Solution::is_additive_number("1991001990".to_owned()), false);
        // assert_eq!(Solution::is_additive_number("1023".to_owned()), false);

        // assert_eq!(Solution::is_additive_number("101".to_owned()), true);
        assert_eq!(Solution::is_additive_number("121474836472147483648".to_owned()), true);
    }
}
