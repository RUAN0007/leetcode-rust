/**
 * [72] Edit Distance
 *
 * Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.
 * You have the following three operations permitted on a word:
 * 
 * 	Insert a character
 * 	Delete a character
 * 	Replace a character
 * 
 *  
 * Example 1:
 * 
 * Input: word1 = "horse", word2 = "ros"
 * Output: 3
 * Explanation: 
 * horse -> rorse (replace 'h' with 'r')
 * rorse -> rose (remove 'r')
 * rose -> ros (remove 'e')
 * 
 * Example 2:
 * 
 * Input: word1 = "intention", word2 = "execution"
 * Output: 5
 * Explanation: 
 * intention -> inention (remove 't')
 * inention -> enention (replace 'i' with 'e')
 * enention -> exention (replace 'n' with 'x')
 * exention -> exection (replace 'n' with 'c')
 * exection -> execution (insert 'u')
 * 
 *  
 * Constraints:
 * 
 * 	0 <= word1.length, word2.length <= 500
 * 	word1 and word2 consist of lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/edit-distance/
// discuss: https://leetcode.com/problems/edit-distance/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 : Vec<char> = word1.chars().collect();
        let word2 : Vec<char> = word2.chars().collect();
        let mut result = vec![vec![0;word2.len()+1];word1.len()+1];
        for i in 0..=word1.len() {
            result[i][0] = i;
        }

        for j in 0..=word2.len() {
            result[0][j] = j;
        }

        for i in 1..=word1.len() {
            for j in 1..=word2.len() {
                result[i][j] = result[i-1][j] + 1;
                result[i][j] = std::cmp::min(result[i][j], result[i][j-1] + 1);
                if word1[i-1] == word2[j-1] {
                    result[i][j] = std::cmp::min(result[i][j], result[i-1][j-1]);
                } else {
                    result[i][j] = std::cmp::min(result[i][j], result[i-1][j-1]+1);
                }
            }
        }
        result[word1.len()][word2.len()] as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_72() {
        assert_eq!(Solution::min_distance("horse".to_owned(), "ros".to_owned()),3);
        assert_eq!(Solution::min_distance("intention".to_owned(), "execution".to_owned()),5);
    }
}
