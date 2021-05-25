/**
 * [472] Concatenated Words
 *
 * Given an array of strings words (without duplicates), return all the concatenated words in the given list of words.
 * A concatenated word is defined as a string that is comprised entirely of at least two shorter words in the given array.
 *  
 * Example 1:
 * 
 * Input: words = ["cat","cats","catsdogcats","dog","dogcatsdog","hippopotamuses","rat","ratcatdogcat"]
 * Output: ["catsdogcats","dogcatsdog","ratcatdogcat"]
 * Explanation: "catsdogcats" can be concatenated by "cats", "dog" and "cats"; 
 * "dogcatsdog" can be concatenated by "dog", "cats" and "dog"; 
 * "ratcatdogcat" can be concatenated by "rat", "cat", "dog" and "cat".
 * Example 2:
 * 
 * Input: words = ["cat","dog","catdog"]
 * Output: ["catdog"]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= words.length <= 10^4
 * 	0 <= words[i].length <= 1000
 * 	words[i] consists of only lowercase English letters.
 * 	0 <= sum(words[i].length) <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/concatenated-words/
// discuss: https://leetcode.com/problems/concatenated-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashSet;
impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(mut words: Vec<String>) -> Vec<String> {
        words.sort_by(|a,b|{a.len().cmp(&b.len())});

        let mut dict : HashSet<String> = HashSet::new();
        let mut result = vec![];
        for word_itr in words.iter() {
            if word_itr.len() != 0 && Self::breakable(word_itr.chars().collect(), &dict) {
                result.push(word_itr.clone());
            }
            dict.insert(word_itr.clone());
        }
        result
    }

    pub fn breakable(word : Vec<char>, dict : &HashSet<String>) -> bool {
        // println!("word={:?}, dict={:?}", word, dict);
        let word_len : usize = word.len();
        let mut breakable_at : Vec<bool> = vec![false; word_len+1];
        breakable_at[0] = true;

        for i in 1..=word_len {
            for j in 0..i {
                let sub_word : String = word.iter().skip(j).take(i-j).cloned().collect();
                if breakable_at[j] && dict.contains(&sub_word) {
                    breakable_at[i] = true;
                    break;
                }
            }
        }
        breakable_at[word_len]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_472() {
        assert_eq!(Solution::find_all_concatenated_words_in_a_dict(vec_string!["cat","cats","catsdogcats","dog","dogcatsdog","hippopotamuses","rat","ratcatdogcat"]), vec_string!["dogcatsdog","catsdogcats","ratcatdogcat"]);

        assert_eq!(Solution::find_all_concatenated_words_in_a_dict(vec_string![""]).len(), 0);
    }
}
