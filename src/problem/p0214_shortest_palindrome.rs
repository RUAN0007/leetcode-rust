/**
 * [214] Shortest Palindrome
 *
 * You are given a string s. You can convert s to a palindrome by adding characters in front of it.
 * Return the shortest palindrome you can find by performing this transformation.
 *  
 * Example 1:
 * Input: s = "aacecaaa"
 * Output: "aaacecaaa"
 * Example 2:
 * Input: s = "abcd"
 * Output: "dcbabcd"
 *  
 * Constraints:
 * 
 * 	0 <= s.length <= 5 * 10^4
 * 	s consists of lowercase English letters only.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-palindrome/
// discuss: https://leetcode.com/problems/shortest-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let chars : Vec<char> = s.chars().collect();
        let n : usize = s.len();
        let mut end_pos : usize = 0; // inclusive
        // find the longest palindrome start at i
        for end in (0..n).rev() {
            let l : usize = end + 1;
            let mut is_palindrome = true;
            for i in 0..=(l/2) {
                if chars[i] != chars[l-1-i] {
                    is_palindrome = false;
                    break
                }
            }
            if is_palindrome {
                end_pos = end;
                break;
            }
        }
        let mut result_prefix : Vec<char> = chars.iter().skip(end_pos+1).cloned().rev().collect();
        result_prefix.extend(chars);
        result_prefix.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_214() {
        assert_eq!(Solution::shortest_palindrome("aacecaaa".to_owned()), "aaacecaaa".to_owned());
        assert_eq!(Solution::shortest_palindrome("abcd".to_owned()), "dcbabcd".to_owned());
    }
}
