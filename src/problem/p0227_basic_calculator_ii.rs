/**
 * [227] Basic Calculator II
 *
 * Given a string s which represents an expression, evaluate this expression and return its value. 
 * The integer division should truncate toward zero.
 * <b data-stringify-type="bold">Note: You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as <code data-stringify-type="code">eval().
 *  
 * Example 1:
 * Input: s = "3+2*2"
 * Output: 7
 * Example 2:
 * Input: s = " 3/2 "
 * Output: 1
 * Example 3:
 * Input: s = " 3+5 / 2 "
 * Output: 5
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 3 * 10^5
 * 	s consists of integers and operators ('+', '-', '*', '/') separated by some number of spaces.
 * 	s represents a valid expression.
 * 	All the integers in the expression are non-negative integers in the range [0, 2^31 - 1].
 * 	The answer is guaranteed to fit in a 32-bit integer.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/basic-calculator-ii/
// discuss: https://leetcode.com/problems/basic-calculator-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    fn process(to_sum : &mut Vec<i32>, last_op : char, last_num : i32) {
        if last_op == '+' {
            to_sum.push(last_num);
        } else if last_op == '-' {
            to_sum.push(0 - last_num);
        } else if last_op == '*' {
            let prev_num : i32 = to_sum.pop().unwrap();
            to_sum.push(prev_num * last_num);
        } else {
            let prev_num : i32 = to_sum.pop().unwrap();
            to_sum.push(prev_num / last_num);
        }
    }

    pub fn calculate(s: String) -> i32 {
        let s : Vec<char> = s.chars().collect();
        let mut to_sum : Vec<i32> = vec![];
        let mut i : usize = 0;
        let mut last_op : char = '+'; 
        let mut last_num : i32 = 0; 
        while i < s.len() {
            if s[i] == ' ' {
                // do nothing
            } else if s[i] == '+' || s[i] == '-' || s[i] == '*' || s[i] == '/' {
                Self::process(&mut to_sum, last_op, last_num);

                last_op = s[i];
                last_num = 0;
            } else {
                last_num = last_num * 10 + ((s[i] as u8 - '0' as u8) as i32);
            }

            i+=1;
        }
        Self::process(&mut to_sum, last_op, last_num);
        to_sum.iter().sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_227() {
        // assert_eq!(Solution::calculate("3+2*2".to_owned()), 7);
        assert_eq!(Solution::calculate(" 3/2 ".to_owned()), 1);
    }
}
