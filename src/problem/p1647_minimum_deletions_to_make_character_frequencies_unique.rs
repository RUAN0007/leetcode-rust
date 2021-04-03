
/**
 * [1647] Minimum Deletions to Make Character Frequencies Unique
 *
 * A string s is called good if there are no two different characters in s that have the same frequency.
 * Given a string s, return the minimum number of characters you need to delete to make s good.
 * The frequency of a character in a string is the number of times it appears in the string. For example, in the string "aab", the frequency of 'a' is 2, while the frequency of 'b' is 1.
 *  
 * Example 1:
 * 
 * Input: s = "aab"
 * Output: 0
 * Explanation: s is already good.
 * 
 * Example 2:
 * 
 * Input: s = "aaabbbcc"
 * Output: 2
 * Explanation: You can delete two 'b's resulting in the good string "aaabcc".
 * Another way it to delete one 'b' and one 'c' resulting in the good string "aaabbc".
 * Example 3:
 * 
 * Input: s = "ceabaacb"
 * Output: 2
 * Explanation: You can delete both 'c's resulting in the good string "eabaab".
 * Note that we only care about characters that are still in the string at the end (i.e. frequency of 0 is ignored).
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^5
 * 	s contains only lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique/
// discuss: https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut char_fqc = HashMap::new();
        s.chars().for_each(|c|{
            if let Some(fqc) = char_fqc.get_mut(&c) {
                *fqc += 1;
            } else {
                char_fqc.insert(c, 1);
            }
        });

        let mut fqc : Vec<usize> = char_fqc.values().map(|c|{*c}).collect();
        fqc.sort();
        let mut last_fqc = 0;
        let mut gaps = vec![];
        let mut result = 0;
        // println!("fqc: {:?}", fqc);
        for i in 0..fqc.len() {
            if last_fqc == fqc[i] {
                if let Some(last_gap) = gaps.pop() {
                    result += fqc[i] - last_gap;
                } else {
                    result += fqc[i]; // decrement to 0
                }
            } else {
                // due to the sort, last_fqc < fqc
                gaps.append(&mut ((last_fqc+1)..(fqc[i])).collect());
                last_fqc = fqc[i];
            }
        }
        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1647() {
        assert_eq!(Solution::min_deletions("aaabbbcc".to_owned()), 2);
        assert_eq!(Solution::min_deletions("aab".to_owned()), 0);
    }
}
