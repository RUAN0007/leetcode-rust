/**
 * [5] Longest Palindromic Substring
 *
 * Given a string s, return the longest palindromic substring in s.
 * 
 *  
 * Example 1:
 * 
 * 
 * Input: s = "babad"
 * Output: "bab"
 * Note: "aba" is also a valid answer.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: s = "cbbd"
 * Output: "bb"
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: s = "a"
 * Output: "a"
 * 
 * 
 * Example 4:
 * 
 * 
 * Input: s = "ac"
 * Output: "a"
 * 
 * 
 *  
 * Constraints:
 * 
 * 
 * 	1 <= s.length <= 1000
 * 	s consist of only digits and English letters (lower-case and/or upper-case),
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindromic-substring/
// discuss: https://leetcode.com/problems/longest-palindromic-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut longest_end_pos = 0usize;
        let mut longest_len = 1usize;
        let n : usize = s.len();
        let mut last_start_positions : Vec<usize> = vec![];

        let s : Vec<char> = s.chars().collect();
        for i in 0..n {
            let mut this_start : Vec<usize> = vec![];
            this_start.push(i);
            if 1 <= i  {
                if s[i-1] == s[i] {
                    this_start.push(i-1);
                }
                for &last_start_position in last_start_positions.iter() {
                    if 1 <= last_start_position && s[last_start_position - 1] == s[i] {
                        this_start.push(last_start_position - 1);
                    }
                }

            }
            for &start_pos in this_start.iter() {
                let palindrome_len : usize = i - start_pos + 1;
                // println!("end(i)={}, start_pos={}, palindrome_len={},longest_len={}", i, start_pos, palindrome_len,longest_len);
                if palindrome_len > longest_len {
                    longest_len = palindrome_len;
                    longest_end_pos = i;
                }
            }
            last_start_positions = this_start; 
        }
        let start_pos : usize = longest_end_pos + 1 - longest_len ; // inclusive
        s.iter().skip(start_pos).take(longest_len).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        assert_eq!(Solution::longest_palindrome("aaaaa".to_owned()), "aaaaa");
        assert_eq!(Solution::longest_palindrome("babab".to_owned()), "babab");
        assert_eq!(Solution::longest_palindrome("babcd".to_owned()), "bab");
        assert_eq!(Solution::longest_palindrome("cbbd".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome("bb".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome("".to_owned()), "");
    }
}
