/**
 * [316] Remove Duplicate Letters
 *
 * Given a string s, remove duplicate letters so that every letter appears once and only once. You must make sure your result is the smallest in lexicographical order among all possible results.
 *  
 * Example 1:
 * 
 * Input: s = "bcabc"
 * Output: "abc"
 * 
 * Example 2:
 * 
 * Input: s = "cbacdcbc"
 * Output: "acdb"
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^4
 * 	s consists of lowercase English letters.
 * 
 *  
 * Note: This question is the same as 1081: <a href="https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/" target="_blank">https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/</a>
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-duplicate-letters/
// discuss: https://leetcode.com/problems/remove-duplicate-letters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let s : Vec<char> = s.chars().collect();
        let mut char_counts : HashMap<char, usize> = HashMap::new();
        for &c in s.iter() {
            *char_counts.entry(c).or_insert(0)+=1;
        }

        let mut stack : Vec<char> = vec![];
        let mut in_stack : HashSet<char> = HashSet::new();
        for &c in s.iter() {
            *char_counts.get_mut(&c).unwrap()-=1;
            if in_stack.contains(&c) {continue;}

            while let Some(&last_char) = stack.last() {
                if (last_char as u8) > (c as u8) && char_counts[&last_char] > 0 {
                    stack.pop();
                    in_stack.remove(&last_char);
                } else {
                    break;
                }
            }

            stack.push(c);
            in_stack.insert(c);
        }
        stack.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_316() {
        assert_eq!(Solution::remove_duplicate_letters("bcabc".to_owned()), "abc".to_owned());

        assert_eq!(Solution::remove_duplicate_letters("cbacdcbc".to_owned()), "acdb".to_owned());

        assert_eq!(Solution::remove_duplicate_letters("bbcaac".to_owned()), "bac".to_owned());
    }
}
