/**
 * [115] Distinct Subsequences
 *
 * Given two strings s and t, return the number of distinct subsequences of s which equals t.
 * A string's subsequence is a new string formed from the original string by deleting some (can be none) of the characters without disturbing the remaining characters' relative positions. (i.e., "ACE" is a subsequence of "ABCDE" while "AEC" is not).
 * It is guaranteed the answer fits on a 32-bit signed integer.
 *  
 * Example 1:
 * 
 * Input: s = "rabbbit", t = "rabbit"
 * Output: 3
 * Explanation:
 * As shown below, there are 3 ways you can generate "rabbit" from S.
 * <u>rabb</u>b<u>it</u>
 * <u>ra</u>b<u>bbit</u>
 * <u>rab</u>b<u>bit</u>
 * 
 * Example 2:
 * 
 * Input: s = "babgbag", t = "bag"
 * Output: 5
 * Explanation:
 * As shown below, there are 5 ways you can generate "bag" from S.
 * <u>ba</u>b<u>g</u>bag
 * <u>ba</u>bgba<u>g</u>
 * <u>b</u>abgb<u>ag</u>
 * ba<u>b</u>gb<u>ag</u>
 * babg<u>bag</u>
 *  
 * Constraints:
 * 
 * 	1 <= s.length, t.length <= 1000
 * 	s and t consist of English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/distinct-subsequences/
// discuss: https://leetcode.com/problems/distinct-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let len_s : usize = s.len();
        let len_t : usize = t.len();
        let s : Vec<char> = s.chars().collect();
        let t : Vec<char> = t.chars().collect();

        let mut result : Vec<Vec<usize>> = vec![vec![0;len_t+1];len_s+1];
        result[0][0] = 1;
        for i in 1..=len_s {
            // empty t match once
            result[i][0] = 1;
        }

        for j in 1..=len_t {
            // empty s match none
            result[0][j] = 0;
        }

        for i in 1..=len_s {
            for j in 1..=len_t {
                if s[i-1] == t[j-1] {
                    result[i][j] = result[i-1][j-1] + result[i-1][j];
                } else {
                    result[i][j] = result[i-1][j];
                }
            }
        }

        result[len_s][len_t] as i32
    }
    pub fn my_num_distinct(s: String, t: String) -> i32 {
        let len_s : usize = s.len();
        let len_t : usize = t.len();
        let s : Vec<char> = s.chars().collect();
        let t : Vec<char> = t.chars().collect();

        let mut result : Vec<Vec<usize>> = vec![vec![0;len_t];len_s];
        let t_last_char : char = *t.last().unwrap();

        let mut count : usize = 0;
        for i in (0..len_s).rev() {
            if s[i] == t_last_char {
                count+=1;
            }
            result[i][len_t - 1] = count;
        }

        for i in (0..len_s).rev() {
            for j in (0..len_t-1).rev() {
                if len_s - i < len_t - j {continue;}
                if s[i] == t[j] {
                    result[i][j] = result[i+1][j+1] + result[i+1][j];
                } else {
                    result[i][j] = result[i+1][j];
                }
            }
        }
        
        result[0][0] as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_115() {
        //assert_eq!(Solution::num_distinct("rabbbit".to_owned(), "rabbit".to_owned()), 3);
        // assert_eq!(
        //     Solution::num_distinct("babgbag".to_owned(), "bag".to_owned()),
        //     5
        // );
        // assert_eq!(
        //     Solution::num_distinct("aaaaaaaaaaaaaaaaaaaa".to_owned(), "aaaaaaaaaa".to_owned()),
        //     184756
        // );
    }
}
