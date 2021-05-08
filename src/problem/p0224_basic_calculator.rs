/**
 * [224] Basic Calculator
 *
 * Given a string s representing an expression, implement a basic calculator to evaluate it.
 *  
 * Example 1:
 * 
 * Input: s = "1 + 1"
 * Output: 2
 * 
 * Example 2:
 * 
 * Input: s = " 2-1 + 2 "
 * Output: 3
 * 
 * Example 3:
 * 
 * Input: s = "(1+(4+5+2)-3)+(6+8)"
 * Output: 23
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 3 * 10^5
 * 	s consists of digits, '+', '-', '(', ')', and ' '.
 * 	s represents a valid expression.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/basic-calculator/
// discuss: https://leetcode.com/problems/basic-calculator/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn recursive_compute(s : &Vec<char>, start : usize, end : usize, level : usize) -> i32 {
        let mut base : i32 = 0;
        let mut prev_op : char = '+';
        let mut prev_num : i32 = 0;

        let mut i : usize = start;
        while i < end {
            // println!("start={}, end = {}, i={}, base={}, prev_op={}, prev_num={}", start, end, i, base, prev_op, prev_num);
            if s[i] == ' ' {
                // do nothing
            } else if s[i] == '+' || s[i] == '-' {
                if prev_op == '+' {
                    base += prev_num;
                } else {
                    base -= prev_num;
                }
                prev_op = s[i];
                prev_num = 0;
            } else if s[i].is_ascii_digit() {
                let digit : i32 = (s[i] as u8 - '0' as u8) as i32;
                prev_num = 10 * prev_num + digit;

            } else if s[i] == '(' {
                let open_pos : usize = i;

                let mut stack : Vec<char> = vec![s[i]];
                loop {
                    i += 1;
                    if s[i] == ')' {
                        while let Some(last_char) = stack.pop() {
                            if last_char == '(' {break}
                        }
                        if stack.len()==0 { break}
                    } else {
                        stack.push(s[i]);
                    }
                }
                let close_pos : usize = i;
                prev_num = Self::recursive_compute(s, open_pos + 1, close_pos, level + 1);
            } else {
                panic!("Unrecognized char {} at {}", s[i], i);
            }
            i += 1;
        }

        if prev_op == '+' {
            base += prev_num;
        } else {
            base -= prev_num;
        }
        base
    }
    pub fn calculate(s: String) -> i32 {
        let s : Vec<char> = s.chars().collect();
        Self::recursive_compute(&s, 0, s.len(), 0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_224() {
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_owned()), 23);
        assert_eq!(Solution::calculate("1+1".to_owned()), 2);
        assert_eq!(Solution::calculate("0".to_owned()), 0);
        assert_eq!(Solution::calculate("2147483647".to_owned()), 2147483647);
    }
}
