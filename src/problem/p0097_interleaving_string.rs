/**
 * [97] Interleaving String
 *
 * Given strings s1, s2, and s3, find whether s3 is formed by an interleaving of s1 and s2.
 * An interleaving of two strings s and t is a configuration where they are divided into non-empty substrings such that:
 * 
 * 	s = s1 + s2 + ... + sn
 * 	t = t1 + t2 + ... + tm
 * 	|n - m| <= 1
 * 	The interleaving is s1 + t1 + s2 + t2 + s3 + t3 + ... or t1 + s1 + t2 + s2 + t3 + s3 + ...
 * 
 * Note: a + b is the concatenation of strings a and b.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/02/interleave.jpg" style="width: 561px; height: 203px;" />
 * Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
 * Output: true
 * 
 * Example 2:
 * 
 * Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
 * Output: false
 * 
 * Example 3:
 * 
 * Input: s1 = "", s2 = "", s3 = ""
 * Output: true
 * 
 *  
 * Constraints:
 * 
 * 	0 <= s1.length, s2.length <= 100
 * 	0 <= s3.length <= 200
 * 	s1, s2, and s3 consist of lowercase English letters.
 * 
 *  
 * Follow up: Could you solve it using only O(s2.length) additional memory space?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/interleaving-string/
// discuss: https://leetcode.com/problems/interleaving-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    fn match_char(s1 : &Vec<char>, s2 : &Vec<char>, s3 : &Vec<char>, s1_pos : usize, s2_pos : usize, s3_pos : usize) -> bool {
        // println!("s1[{}]={},s2[{}]={},s3[{}]={}", s1_pos, s1[s1_pos], s2_pos, s1[s2_pos], s3_pos, s3[s3_pos]);
        println!("s1[{}],s2[{}],s3[{}]", s1_pos, s2_pos, s3_pos);
        if s3_pos == s3.len() {return true;}


        if s1_pos < s1.len() && s2_pos < s2.len() && s3[s3_pos] == s1[s1_pos] && s3[s3_pos] == s2[s2_pos] {
           Self::match_char(s1, s2, s3, s1_pos+1, s2_pos, s3_pos+1) ||
           Self::match_char(s1, s2, s3, s1_pos, s2_pos+1, s3_pos+1)
        } else if s1_pos < s1.len() && s3[s3_pos] == s1[s1_pos] {
           Self::match_char(s1, s2, s3, s1_pos+1, s2_pos, s3_pos+1)
        } else if s2_pos < s2.len() && s3[s3_pos] == s2[s2_pos] {
           Self::match_char(s1, s2, s3, s1_pos, s2_pos+1, s3_pos+1)
        } else {
           false
        }
    }

    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let s1 : Vec<char> = s1.chars().collect();
        let s2 : Vec<char> = s2.chars().collect();
        let s3 : Vec<char> = s3.chars().collect();
        Self::match_char(&s1, &s2, &s3, 0, 0, 0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_97() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_owned(),
                "dbbca".to_owned(),
                "aadbbcbcac".to_owned()
            ),
            true
        );
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_owned(),
                "dbbca".to_owned(),
                "aadbbbaccc".to_owned()
            ),
            false
        );
        assert_eq!(
            Solution::is_interleave("a".to_owned(), "b".to_owned(), "a".to_owned()),
            false
        );

        assert_eq!(
            Solution::is_interleave("aa".to_owned(), "aa".to_owned(), "aaaa".to_owned()),
           true 
        );
    }
}
