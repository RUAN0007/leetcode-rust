/**
 * [139] Word Break
 *
 * Given a string s and a dictionary of strings wordDict, return true if s can be segmented into a space-separated sequence of one or more dictionary words.
 * Note that the same word in the dictionary may be reused multiple times in the segmentation.
 *  
 * Example 1:
 * 
 * Input: s = "leetcode", wordDict = ["leet","code"]
 * Output: true
 * Explanation: Return true because "leetcode" can be segmented as "leet code".
 * 
 * Example 2:
 * 
 * Input: s = "applepenapple", wordDict = ["apple","pen"]
 * Output: true
 * Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
 * Note that you are allowed to reuse a dictionary word.
 * 
 * Example 3:
 * 
 * Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 300
 * 	1 <= wordDict.length <= 1000
 * 	1 <= wordDict[i].length <= 20
 * 	s and wordDict[i] consist of only lowercase English letters.
 * 	All the strings of wordDict are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-break/
// discuss: https://leetcode.com/problems/word-break/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let l : usize = s.len();
        let dict : HashSet<String> = word_dict.into_iter().collect();
        let mut breakable : Vec<bool> = vec![false;l+1];
        breakable[0] = true;

        for i in 1..=l {
            for start in 0..i {
                let sub_str : String = s[start..i].to_string();
                // println!("i={}, start={}, sub_str={}", i, start, sub_str);
                if breakable[start] && dict.contains(&sub_str) {
                    breakable[i] = true;
                    break;
                }
            }
        }

        breakable[l] 
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_139() {
        assert_eq!(
            Solution::word_break("leetcode".to_owned(), vec_string!["leet", "code"]),
            true
        );
        assert_eq!(
            Solution::word_break(
                "catsandog".to_owned(),
                vec_string!["cats", "dog", "sand", "and", "cat"]
            ),
            false
        );
    }
}
