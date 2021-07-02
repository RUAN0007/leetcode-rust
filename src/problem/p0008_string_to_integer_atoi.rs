/**
 * [8] String to Integer (atoi)
 *
 * Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer (similar to C/C++'s atoi function).
 * The algorithm for myAtoi(string s) is as follows:
 * <ol>
 * 	Read in and ignore any leading whitespace.
 * 	Check if the next character (if not already at the end of the string) is '-' or '+'. Read this character in if it is either. This determines if the final result is negative or positive respectively. Assume the result is positive if neither is present.
 * 	Read in next the characters until the next non-digit charcter or the end of the input is reached. The rest of the string is ignored.
 * 	Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read, then the integer is 0. Change the sign as necessary (from step 2).
 * 	If the integer is out of the 32-bit signed integer range [-2^31, 2^31 - 1], then clamp the integer so that it remains in the range. Specifically, integers less than -2^31 should be clamped to -2^31, and integers greater than 2^31 - 1 should be clamped to 2^31 - 1.
 * 	Return the integer as the final result.
 * </ol>
 * Note:
 * 
 * 	Only the space character ' ' is considered a whitespace character.
 * 	Do not ignore any characters other than the leading whitespace or the rest of the string after the digits.
 * 
 *  
 * Example 1:
 * 
 * Input: s = "42"
 * Output: 42
 * Explanation: The underlined characters are what is read in, the caret is the current reader position.
 * Step 1: "42" (no characters read because there is no leading whitespace)
 *          ^
 * Step 2: "42" (no characters read because there is neither a '-' nor '+')
 *          ^
 * Step 3: "<u>42</u>" ("42" is read in)
 *            ^
 * The parsed integer is 42.
 * Since 42 is in the range [-2^31, 2^31 - 1], the final result is 42.
 * 
 * Example 2:
 * 
 * Input: s = "   -42"
 * Output: -42
 * Explanation:
 * Step 1: "<u>   </u>-42" (leading whitespace is read and ignored)
 *             ^
 * Step 2: "   <u>-</u>42" ('-' is read, so the result should be negative)
 *              ^
 * Step 3: "   -<u>42</u>" ("42" is read in)
 *                ^
 * The parsed integer is -42.
 * Since -42 is in the range [-2^31, 2^31 - 1], the final result is -42.
 * 
 * Example 3:
 * 
 * Input: s = "4193 with words"
 * Output: 4193
 * Explanation:
 * Step 1: "4193 with words" (no characters read because there is no leading whitespace)
 *          ^
 * Step 2: "4193 with words" (no characters read because there is neither a '-' nor '+')
 *          ^
 * Step 3: "<u>4193</u> with words" ("4193" is read in; reading stops because the next character is a non-digit)
 *              ^
 * The parsed integer is 4193.
 * Since 4193 is in the range [-2^31, 2^31 - 1], the final result is 4193.
 * 
 * Example 4:
 * 
 * Input: s = "words and 987"
 * Output: 0
 * Explanation:
 * Step 1: "words and 987" (no characters read because there is no leading whitespace)
 *          ^
 * Step 2: "words and 987" (no characters read because there is neither a '-' nor '+')
 *          ^
 * Step 3: "words and 987" (reading stops immediately because there is a non-digit 'w')
 *          ^
 * The parsed integer is 0 because no digits were read.
 * Since 0 is in the range [-2^31, 2^31 - 1], the final result is 0.
 * 
 * Example 5:
 * 
 * Input: s = "-91283472332"
 * Output: -2147483648
 * Explanation:
 * Step 1: "-91283472332" (no characters read because there is no leading whitespace)
 *          ^
 * Step 2: "<u>-</u>91283472332" ('-' is read, so the result should be negative)
 *           ^
 * Step 3: "-<u>91283472332</u>" ("91283472332" is read in)
 *                      ^
 * The parsed integer is -91283472332.
 * Since -91283472332 is less than the lower bound of the range [-2^31, 2^31 - 1], the final result is clamped to -2^31 = -2147483648.<span style="display: none;"> </span>
 * 
 *  
 * Constraints:
 * 
 * 	0 <= s.length <= 200
 * 	s consists of English letters (lower-case and upper-case), digits (0-9), ' ', '+', '-', and '.'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/string-to-integer-atoi/
// discuss: https://leetcode.com/problems/string-to-integer-atoi/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s : Vec<char> = s.chars().collect();
        // trim empty prefix
        let mut prefix_space_count : usize = 0;
        for (i, &c) in s.iter().enumerate() {
            if c != ' ' {
                prefix_space_count = i;
                break;
            }
        }
        let s : Vec<char> = s[prefix_space_count..].iter().cloned().collect();
        if s.len() == 0 {return 0}

        let mut i : usize = 0;
        let mut signed : i32 = 1;
        if s[0] == '+' {
            signed = 1;
            i+=1;
        } else if s[0] == '-' {
            signed = -1;
            i+=1;
        } else if !s[0].is_ascii_digit() {
            return 0;
        }

        let mut result : i32 = 0;
        while i < s.len() && s[i].is_ascii_digit() {
            // println!("i={},result={},s[i]={}",i, result,s[i]);
            if let Some(r) = result.checked_mul(10i32) {
                result = r;
            } else if signed == 1 {
                return 2_147_483_647i32;
            } else {
                return -2_147_483_648i32;
            }

            if let Some(r) = result.checked_add((s[i] as u8 - '0' as u8) as i32) {
                result = r;
            } else if signed == 1 {
                return 2_147_483_647i32;
            } else {
                return -2_147_483_648i32;
            }

            if let Some(r) = result.checked_mul(signed) {
                // result = r;
            } else if signed == 1 {
                return 2_147_483_647i32;
            } else {
                return -2_147_483_648i32;
            }
            i+=1;
        }
        result * signed
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8() {
        assert_eq!(Solution::my_atoi("aa".to_string()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
        assert_eq!(Solution::my_atoi("004193333".to_string()), 4193333);
    }
}
