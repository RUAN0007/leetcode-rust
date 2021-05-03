/**
 * [65] Valid Number
 *
 * A valid number can be split up into these components (in order):
 * <ol>
 * 	A decimal number or an integer.
 * 	(Optional) An 'e' or 'E', followed by an integer.
 * </ol>
 * A decimal number can be split up into these components (in order):
 * <ol>
 * 	(Optional) A sign character (either '+' or '-').
 * 	One of the following formats:
 * 	<ol>
 * 		At least one digit, followed by a dot '.'.
 * 		At least one digit, followed by a dot '.', followed by at least one digit.
 * 		A dot '.', followed by at least one digit.
 * 	</ol>
 * 	
 * </ol>
 * An integer can be split up into these components (in order):
 * <ol>
 * 	(Optional) A sign character (either '+' or '-').
 * 	At least one digit.
 * </ol>
 * For example, all the following are valid numbers: ["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"], while the following are not valid numbers: ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"].
 * Given a string s, return true if s is a valid number.
 *  
 * Example 1:
 * 
 * Input: s = "0"
 * Output: true
 * 
 * Example 2:
 * 
 * Input: s = "e"
 * Output: false
 * 
 * Example 3:
 * 
 * Input: s = "."
 * Output: false
 * 
 * Example 4:
 * 
 * Input: s = ".1"
 * Output: true
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 20
 * 	s consists of only English letters (both uppercase and lowercase), digits (0-9), plus '+', minus '-', or dot '.'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-number/
// discuss: https://leetcode.com/problems/valid-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_integer(s : &String, min_numeric_len: usize, sign_optional : bool) -> bool {
        if s.len() == 0 {
            return min_numeric_len == 0;
        }

        let mut s : Vec<char> = s.chars().collect();
        if sign_optional && (s[0] == '+' || s[0] == '-') {
            s = s.iter().skip(1).cloned().collect();
        }

        let mut valid : bool = s.len() >= min_numeric_len;
        for &c in s.iter() {
            if !c.is_ascii_digit() {
                valid = false;
                break;
            }
        }
        valid
    }

    pub fn is_decimal(s : &String) -> bool {
        let s : Vec<char> = s.chars().collect();
        if let Some(dot_pos) = s.iter().position(|c : &char|{*c=='.'}) {
            let first_part : String = s.iter().take(dot_pos).collect();
            let sec_part : String = s.iter().skip(dot_pos + 1).collect();
            println!("\tfirst_part_decimal = [{}], sec_part_decimal = [{}]", first_part, sec_part);
            let ge1_char_first = Self::is_integer(&first_part, 1, true);
            let ge0_char_sec = Self::is_integer(&sec_part, 0, false);

            let ge0_char_first = Self::is_integer(&first_part, 0, true);
            let ge1_char_sec = Self::is_integer(&sec_part, 1, false);
            println!("ge1_char_first={}, ge0_char_sec={}",ge1_char_first, ge0_char_sec);
            println!("ge0_char_first={}, ge1_char_sec={}",ge0_char_first, ge1_char_sec);
            (ge1_char_first  && ge0_char_sec) || (ge0_char_first && ge1_char_sec)
        } else {
            false
        }
    }

    pub fn is_number(s: String) -> bool {
        let s : Vec<char> = s.chars().collect();
        if let Some(e_pos) = s.iter().position(|c : &char|{*c=='E' || *c =='e'}) {
            let first_part : String = s.iter().take(e_pos).collect();
            let sec_part : String = s.iter().skip(e_pos + 1).collect();
            println!("first_part = {}, sec_part = {}", first_part, sec_part);

            let mut valid_first : bool = false;
            let mut valid_sec : bool = false;
            
            if Self::is_decimal(&first_part) {
                println!("first part is decimal");
                valid_first = true;
            }

            if !valid_first && Self::is_integer(&first_part, 1, true) {
                println!("first part is int.");
                valid_first = true;
            }

            if Self::is_integer(&sec_part, 1, true) {
                println!("sec part is int.");
                valid_sec = true;
            }
            valid_first && valid_sec
        } else {
            let s : String = s.iter().collect();
            let mut valid_whole : bool = false;
            if Self::is_decimal(&s) {
                println!("The whole is decimal");
                valid_whole = true;
            }

            if !valid_whole && Self::is_integer(&s, 1, true) {
                println!("The whole is int.");
                valid_whole = true;
            }
            valid_whole
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_65() {
        assert!(Solution::is_number("0".to_owned()));
        assert!(!Solution::is_number("e".to_owned()));
        assert!(!Solution::is_number(".".to_owned()));
        assert!(Solution::is_number(".1".to_owned()));
        assert!(Solution::is_number("2e0".to_owned()));
    }
}
