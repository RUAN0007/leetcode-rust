/**
 * [140] Word Break II
 *
 * Given a string s and a dictionary of strings wordDict, add spaces in s to construct a sentence where each word is a valid dictionary word. Return all such possible sentences in any order.
 * Note that the same word in the dictionary may be reused multiple times in the segmentation.
 *  
 * Example 1:
 * 
 * Input: s = "catsanddog", wordDict = ["cat","cats","and","sand","dog"]
 * Output: ["cats and dog","cat sand dog"]
 * 
 * Example 2:
 * 
 * Input: s = "pineapplepenapple", wordDict = ["apple","pen","applepen","pine","pineapple"]
 * Output: ["pine apple pen apple","pineapple pen apple","pine applepen apple"]
 * Explanation: Note that you are allowed to reuse a dictionary word.
 * 
 * Example 3:
 * 
 * Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
 * Output: []
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 20
 * 	1 <= wordDict.length <= 1000
 * 	1 <= wordDict[i].length <= 10
 * 	s and wordDict[i] consist of only lowercase English letters.
 * 	All the strings of wordDict are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-break-ii/
// discuss: https://leetcode.com/problems/word-break-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashSet;
impl Solution {
    pub fn recursive(result : &mut Vec<Vec<String>>, tmp: &mut Vec<String>, s : &String, cur_idx : usize, word_dict : &HashSet<String>, level : usize) {
        let pad :String = (0..level).map(|_|{"  "}).collect();
        // println!("{}tmp={:?}, cur_idx={}", pad, tmp, cur_idx);
        let n : usize = s.len();
        if n == cur_idx {
            result.push(tmp.clone());
            return;
        }

        // exclusive end
        for end in (cur_idx+1)..=n {
            let sub_str : String = s[cur_idx..end].to_owned();
            if word_dict.contains(&sub_str) {
                tmp.push(sub_str);
                Self::recursive(result, tmp, s, end, word_dict, level + 1);
                tmp.pop();
            }
        }
    }



    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut result : Vec<Vec<String>> = vec![];
        let mut tmp : Vec<String> = vec![];
        let word_dict : HashSet<String> = word_dict.into_iter().collect();
        Self::recursive(&mut result, &mut tmp, &s, 0, &word_dict, 0);
        result.iter().map(|v|{v.join(" ")}).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_140() {
        assert_eq!(Solution::word_break("catsanddog".to_owned(), vec!["cat".to_owned(),"cats".to_owned(),"and".to_owned(),"sand".to_owned(),"dog".to_owned()]), vec!["cat sand dog".to_owned(),"cats and dog".to_owned()]);
    }
}
