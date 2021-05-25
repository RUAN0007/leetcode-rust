/**
 * [466] Count The Repetitions
 *
 * We define str = [s, n] as the string str which consists of the string s concatenated n times.
 * 
 * 	For example, str == ["abc", 3] =="abcabcabc".
 * 
 * We define that string s1 can be obtained from string s2 if we can remove some characters from s2 such that it becomes s1.
 * 
 * 	For example, s1 = "abc" can be obtained from s2 = "ab<u>dbe</u>c" based on our definition by removing the bolded underlined characters.
 * 
 * You are given two strings s1 and s2 and two integers n1 and n2. You have the two strings str1 = [s1, n1] and str2 = [s2, n2].
 * Return the maximum integer m such that str = [str2, m] can be obtained from str1.
 *  
 * Example 1:
 * Input: s1 = "acb", n1 = 4, s2 = "ab", n2 = 2
 * Output: 2
 * Example 2:
 * Input: s1 = "acb", n1 = 1, s2 = "acb", n2 = 1
 * Output: 1
 *  
 * Constraints:
 * 
 * 	1 <= s1.length, s2.length <= 100
 * 	s1 and s2 consist of lowercase English letters.
 * 	1 <= n1, n2 <= 10^6
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-the-repetitions/
// discuss: https://leetcode.com/problems/count-the-repetitions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let s1 : Vec<char> = s1.chars().collect();
        let s2 : Vec<char> = s2.chars().collect();

        let mut count1 : i32 = 0;
        let mut count2 : i32 = 0;
        let mut i1 : usize = 0;
        let mut i2 : usize = 0;
        let mut len1 : usize = s1.len();
        let mut len2 : usize = s2.len();

        while count1 < n1 {
            if s1[i1] == s2[i2] {
                i2+=1;
                if i2==len2 {
                    i2 = 0;
                    count2+=1;
                }
            }

            i1+=1;
            if i1 == len1 {
                i1 = 0;
                count1+=1;
            }
        }
        count2 / n2
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_466() {
        assert_eq!(Solution::get_max_repetitions("acb".to_owned(), 4, "ab".to_owned(), 2), 2);

        assert_eq!(Solution::get_max_repetitions("acb".to_owned(), 1, "acb".to_owned(), 1), 1);
    }
}
