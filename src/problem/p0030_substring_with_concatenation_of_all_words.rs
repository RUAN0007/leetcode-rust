/**
 * [30] Substring with Concatenation of All Words
 *
 * You are given a string s and an array of strings words of the same length. Return all starting indices of substring(s) in s that is a concatenation of each word in words exactly once, in any order, and without any intervening characters.
 * You can return the answer in any order.
 *  
 * Example 1:
 * 
 * Input: s = "barfoothefoobarman", words = ["foo","bar"]
 * Output: [0,9]
 * Explanation: Substrings starting at index 0 and 9 are "barfoo" and "foobar" respectively.
 * The output order does not matter, returning [9,0] is fine too.
 * 
 * Example 2:
 * 
 * Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
 * Output: []
 * 
 * Example 3:
 * 
 * Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
 * Output: [6,9,12]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^4
 * 	s consists of lower-case English letters.
 * 	1 <= words.length <= 5000
 * 	1 <= words[i].length <= 30
 * 	words[i] consists of lower-case English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/substring-with-concatenation-of-all-words/
// discuss: https://leetcode.com/problems/substring-with-concatenation-of-all-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let substr_len : usize = words[0].len();
        let str_len : usize = s.len();
        let s : Vec<char> = s.chars().collect(); // into char vec for random access
        let substr_count : usize = str_len - substr_len + 1;
        let mut matches : Vec<i32> = vec![-1;substr_count];
        for i in 0..substr_count {
            let sub : String = s[i..i+substr_len].iter().collect();
            for (j, word) in words.iter().enumerate() {
                if word.eq(&sub) {
                    matches[i] = j as i32;
                }
            }
        }

        // println!("matches = {:?}", matches);

        let mut result : Vec<i32> = vec![];
        let concat_count : usize = (str_len - substr_len * words.len() + 1) as usize;

        let mut default_needed_count : HashMap<String, usize> = HashMap::new();
        for word in words.iter() {
            *default_needed_count.entry(word.clone()).or_insert(0) +=1;
        }

        for i in 0..concat_count {
            let mut j : usize = i;
            let mut needed_count = default_needed_count.clone();
            while j < substr_count && matches[j] != -1 {
                if let Some(count_ptr) = needed_count.get_mut(&words[matches[j] as usize]) {
                    if *count_ptr == 0 {
                        break;
                    } else {
                        *count_ptr-=1;
                    }
                }

                j += substr_len;
            }
            if needed_count.values().sum::<usize>() == 0 {
                result.push(i as i32);
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_30() {
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ]
            ),
            vec![8]
        );
        assert_eq!(
            Solution::find_substring(
                "barfoothefoobarman".to_string(),
                vec!["foo".to_string(), "bar".to_string()]
            ),
            vec![0, 9]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "word".to_string()
                ]
            ),
            vec![]
        );
        assert_eq!(
            Solution::find_substring(
                "xxwordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ]
            ),
            vec![10]
        );
    }
}
