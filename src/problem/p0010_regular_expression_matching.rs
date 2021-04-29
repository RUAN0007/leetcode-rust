/**
 * [10] Regular Expression Matching
 *
 * Given an input string (s) and a pattern (p), implement regular expression matching with support for '.' and '*' where: 
 * 
 * 	'.' Matches any single character.​​​​
 * 	'*' Matches zero or more of the preceding element.
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
 * Input: s = "aa", p = "a*"
 * Output: true
 * Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
 * 
 * Example 3:
 * 
 * Input: s = "ab", p = ".*"
 * Output: true
 * Explanation: ".*" means "zero or more (*) of any character (.)".
 * 
 * Example 4:
 * 
 * Input: s = "aab", p = "c*a*b"
 * Output: true
 * Explanation: c can be repeated 0 times, a can be repeated 1 time. Therefore, it matches "aab".
 * 
 * Example 5:
 * 
 * Input: s = "mississippi", p = "mis*is*p*."
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	0 <= s.length <= 20
 * 	0 <= p.length <= 30
 * 	s contains only lowercase English letters.
 * 	p contains only lowercase English letters, '.', and '*'.
 * 	It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/regular-expression-matching/
// discuss: https://leetcode.com/problems/regular-expression-matching/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut pattern : Vec<(char, bool)> = vec![]; // bool indicate wether it is repeatable.
        let mut i : usize = 0;
        let s : Vec<char> = s.chars().collect(); // into a vector for random access
        let p : Vec<char> = p.chars().collect(); // into a vector for random access
        while i < p.len() {
            let this_char : char = p[i];
            let mut repeatable : bool = false;
            if i < p.len() - 1 && p[i+1] == '*' {
                repeatable = true;
                i+=1;
            }
            pattern.push((this_char, repeatable));
            i+=1;
        }
        // println!("pattern: {:?}", pattern);

        // "" can match any repeatable patterns
        let s_len : usize = s.len();
        let pattern_len : usize = pattern.len();
        let mut matches : Vec<Vec<bool>> = vec![vec![false; pattern_len + 1];s_len + 1];
        matches[0][0] = true;
        for (i, sub_pattern) in pattern.iter().enumerate() {
            if sub_pattern.1 {
                matches[0][i+1] = true;
            } else {
                break;
            }
        }

        for i in 1..=s_len {
            let cur_str_char : char = s[i-1];
            for j in 1..=pattern_len {
                let cur_pattern_char : char = pattern[j-1].0;
                let cur_repeatable : bool = pattern[j-1].1;
                if cur_repeatable {
                    if cur_str_char == cur_pattern_char || cur_pattern_char == '.' {
                        // match * to no char or cur char. 
                        matches[i][j] = matches[i][j-1] || matches[i-1][j];
                    } else {
                        // match * to no char
                        matches[i][j] = matches[i][j-1];
                    }

                } else {
                    if cur_str_char == cur_pattern_char || cur_pattern_char == '.' {
                        matches[i][j] = matches[i-1][j-1];
                    } else {
                        matches[i][j] = false;
                    }
                }

            }
        }

        // for i in 0..=s_len {
        //     println!("{:?}", matches[i]);
        // }

        matches[s_len][pattern_len]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10() {
        assert!(Solution::is_match("aab".to_owned(), "c*a*b".to_owned()));
        assert!(Solution::is_match("ab".to_owned(), ".*".to_owned()));
        assert!(!Solution::is_match("aa".to_owned(), "a".to_owned()));
        assert!(Solution::is_match("aa".to_owned(), "a*".to_owned()));
        assert!(!Solution::is_match("mississippi".to_owned(), "mis*is*p*.".to_owned()));
    }
}
