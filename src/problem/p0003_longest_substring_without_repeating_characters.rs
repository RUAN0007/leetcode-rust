/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string s, find the length of the longest substring without repeating characters.
 *  
 * Example 1:
 * 
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 * 
 * Example 2:
 * 
 * Input: s = "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 * 
 * Example 3:
 * 
 * Input: s = "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 * Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
 * 
 * Example 4:
 * 
 * Input: s = ""
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	0 <= s.length <= 5 * 10^4
 * 	s consists of English letters, digits, symbols and spaces.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/
// discuss: https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut left, mut right) = (0usize, 0usize);
        // The index of each char of current substr, 
        let mut exists: HashMap<char, usize> = HashMap::new();
        let mut max_len = 0;

        while let Some(right_char) = s.chars().nth(right) {
            if let Some(&dup_pos) = exists.get(&right_char) {
                // Identify a valid str, which starts at left inclusive and ends at right exclusive. 
                max_len = std::cmp::max(max_len, right - left);

                // Remove chars from left to dup_idx in exists.
                for i in left..=dup_pos {
                    if let None = exists.remove(&s.chars().nth(i).unwrap()) {
                        panic!("Not found '{}' at exists when left = {}, right = {}", s.chars().nth(i).unwrap(), left, right);
                    }
                }
                left = dup_pos + 1; // left can be equal to right, which implies empty substr. 
            }
            exists.insert(right_char, right);
            // *exists.entry(right_char).or_insert().unwrap() = right;
            right+=1;
        }

        // now right points the end of str. 
        max_len = std::cmp::max(max_len, right - left);
        max_len as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("bbbb".to_string()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}
