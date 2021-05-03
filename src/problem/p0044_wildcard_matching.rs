/**
 * [44] Wildcard Matching
 *
 * Given an input string (s) and a pattern (p), implement wildcard pattern matching with support for '?' and '*' where:
 * 
 * 	'?' Matches any single character.
 * 	'*' Matches any sequence of characters (including the empty sequence).
 * 
 * The matching should cover the entire input string (not partial).
 *  
 * Example 1:
 * 
 * Input: s = "aa", p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 * 
 * Example 2:
 * 
 * Input: s = "aa", p = "*"
 * Output: true
 * Explanation: '*' matches any sequence.
 * 
 * Example 3:
 * 
 * Input: s = "cb", p = "?a"
 * Output: false
 * Explanation: '?' matches 'c', but the second letter is 'a', which does not match 'b'.
 * 
 * Example 4:
 * 
 * Input: s = "adceb", p = "*a*b"
 * Output: true
 * Explanation: The first '*' matches the empty sequence, while the second '*' matches the substring "dce".
 * 
 * Example 5:
 * 
 * Input: s = "acdcb", p = "a*c?b"
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	0 <= s.length, p.length <= 2000
 * 	s contains only lowercase English letters.
 * 	p contains only lowercase English letters, '?' or '*'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/wildcard-matching/
// discuss: https://leetcode.com/problems/wildcard-matching/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s : Vec<char> = s.chars().collect();
        let p : Vec<char> = p.chars().collect();
        let mut matches = vec![vec![false;p.len()+1];s.len()+1];
        matches[0][0] = true;
        for j in 1..=p.len() {
            if p[j-1] == '*' {
                matches[0][j] = true;
            } else {
                break;
            }
        }

        for i in 1..=s.len() {
            for j in 1..=p.len() {
                if p[j-1] == '?' {
                    matches[i][j] = matches[i-1][j-1];
                } else if p[j-1] == '*' {
                    matches[i][j] = matches[i][j-1] || matches[i-1][j];
                } else {
                    matches[i][j] = matches[i-1][j-1] && s[i-1] == p[j-1];
                }
            }
        }
        matches[s.len()][p.len()]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_44() {
        assert!(!Solution::is_match("aa".to_owned(), "a".to_owned()));
        assert!(Solution::is_match("aa".to_owned(), "*".to_owned()));
        assert!(!Solution::is_match("cb".to_owned(), "*a".to_owned()));
        assert!(Solution::is_match("adceb".to_owned(), "*a*b".to_owned()));
        assert!(!Solution::is_match("acdcb".to_owned(), "a*c?b".to_owned()));
    }
}
