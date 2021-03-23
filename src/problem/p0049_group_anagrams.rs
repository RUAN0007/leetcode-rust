/**
 * [49] Group Anagrams
 *
 * Given an array of strings strs, group the anagrams together. You can return the answer in any order.
 * An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
 *  
 * Example 1:
 * Input: strs = ["eat","tea","tan","ate","nat","bat"]
 * Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
 * Example 2:
 * Input: strs = [""]
 * Output: [[""]]
 * Example 3:
 * Input: strs = ["a"]
 * Output: [["a"]]
 *  
 * Constraints:
 * 
 * 	1 <= strs.length <= 10^4
 * 	0 <= strs[i].length <= 100
 * 	strs[i] consists of lower-case English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/group-anagrams/
// discuss: https://leetcode.com/problems/group-anagrams/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut dedup: HashMap<String, Vec<String>> =  HashMap::new();
        for str in strs {
            let mut chars: Vec<char> = str.chars().collect();
            chars.sort_by(|a, b| a.cmp(b));
            let s : String = chars.into_iter().collect();
            if let Some(x) = dedup.get_mut(&s) {
                x.push(str);
            } else {
                dedup.insert(s, vec![str]);
            }
        }
        let mut result = vec![];

        for (_, r) in dedup {
            result.push(r);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;
    // TODO: implement arbitrary match macro
    #[test]
    #[ignore]
    fn test_49() {
        assert_eq!(
            Solution::group_anagrams(vec_string!["eat", "tea", "tan", "ate", "nat", "bat"]),
            vec![
                vec_string!["tan", "nat"],
                vec_string!["bat"],
                vec_string!["eat", "ate", "tea"],
            ]
        );
    }
}
