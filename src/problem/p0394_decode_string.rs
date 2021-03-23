use futures::io::repeat;

/**
 * [394] Decode String
 *
 * Given an encoded string, return its decoded string.
 * The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.
 * You may assume that the input string is always valid; No extra white spaces, square brackets are well-formed, etc.
 * Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. For example, there won't be input like 3a or 2[4].
 *  
 * Example 1:
 * Input: s = "3[a]2[bc]"
 * Output: "aaabcbc"
 * Example 2:
 * Input: s = "3[a2[c]]"
 * Output: "accaccacc"
 * Example 3:
 * Input: s = "2[abc]3[cd]ef"
 * Output: "abcabccdcdcdef"
 * Example 4:
 * Input: s = "abc3[cd]xyz"
 * Output: "abccdcdcdxyz"
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 30
 * 	s consists of lowercase English letters, digits, and square brackets '[]'.
 * 	s is guaranteed to be a valid input.
 * 	All the integers in s are in the range [1, 300].
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/decode-string/
// discuss: https://leetcode.com/problems/decode-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack = vec![];
        let mut result = String::from("");
        for c in s.chars() {
            println!("Char = {}", c);
            if c.is_ascii_digit() {
                stack.push(c.to_string());
            } else if c == ']' {
                let mut raw_str = String::from("");
                while let Some(last) = stack.pop() {
                    // print!("\tStack Pop: {}", last);
                    if last == "[" {
                        break;
                    }
                    raw_str = last + raw_str.as_str();
                }

                let mut raw_digits = String::from("");
                while let Some(last) = stack.last(){
                    if last.chars().nth(0).unwrap().is_ascii_digit() {
                        raw_digits = last.clone() + raw_digits.as_str();
                        stack.pop();
                    } else {
                        break
                    }
                }
                let count = raw_digits.parse::<i32>().unwrap();
                let repeated = raw_str.repeat(count as usize);
                println!("Repeated: {}", repeated);

                if stack.is_empty() {
                    result = result + repeated.as_str();
                } else {
                    stack.push(repeated);
                }
            } else if stack.is_empty() {
                result.push(c);
            } else {
                stack.push(c.to_string());
            }
            println!("Stack = {:?}", stack);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_394() {
        assert_eq!(Solution::decode_string("3[a2[c]]".to_string()), "accaccacc");

    }
}
