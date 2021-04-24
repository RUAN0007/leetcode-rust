/**
 * [1143] Longest Common Subsequence
 *
 * Given two strings text1 and text2, return the length of their longest common subsequence. If there is no common subsequence, return 0.
 * A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
 * 
 * 	For example, "ace" is a subsequence of "abcde".
 * 
 * A common subsequence of two strings is a subsequence that is common to both strings.
 *  
 * Example 1:
 * 
 * Input: text1 = "abcde", text2 = "ace" 
 * Output: 3  
 * Explanation: The longest common subsequence is "ace" and its length is 3.
 * 
 * Example 2:
 * 
 * Input: text1 = "abc", text2 = "abc"
 * Output: 3
 * Explanation: The longest common subsequence is "abc" and its length is 3.
 * 
 * Example 3:
 * 
 * Input: text1 = "abc", text2 = "def"
 * Output: 0
 * Explanation: There is no such common subsequence, so the result is 0.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= text1.length, text2.length <= 1000
 * 	text1 and text2 consist of only lowercase English characters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-common-subsequence/
// discuss: https://leetcode.com/problems/longest-common-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1 : Vec<char> = text1.chars().collect();
        let text2 : Vec<char> = text2.chars().collect();
        let n1 : usize = text1.len();
        let n2 : usize = text2.len();
        let mut result = vec![vec![0i32;n2 + 1];n1+1];
        for i in 1..=n1 {
            for j in 1..=n2 {
                if text1[i-1] == text2[j-1] {
                    result[i][j] = std::cmp::max(result[i][j], result[i-1][j-1]+1);
                } else {
                    result[i][j] = std::cmp::max(result[i][j], result[i][j-1]);
                    result[i][j] = std::cmp::max(result[i][j], result[i-1][j]);
                }
            }
        }
        result[n1][n2]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1143() {
        assert_eq!(Solution::longest_common_subsequence( "abcde".to_owned(), "ace".to_owned()), 3);
        assert_eq!(Solution::longest_common_subsequence( "abc".to_owned(), "abc".to_owned()), 3);
        assert_eq!(Solution::longest_common_subsequence( "abc".to_owned(), "def".to_owned()), 0);
    }
}
