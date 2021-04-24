/**
 * [1092] Shortest Common Supersequence 
 *
 * Given two strings str1 and str2, return the shortest string that has both str1 and str2 as subsequences.  If multiple answers exist, you may return any of them.
 * (A string S is a subsequence of string T if deleting some number of characters from T (possibly 0, and the characters are chosen <u>anywhere</u> from T) results in the string S.)
 *  
 * Example 1:
 * 
 * Input: str1 = <span id="example-input-1-1">"abac"</span>, str2 = <span id="example-input-1-2">"cab"</span>
 * Output: <span id="example-output-1">"cabac"</span>
 * Explanation: 
 * str1 = "abac" is a subsequence of "cabac" because we can delete the first "c".
 * str2 = "cab" is a subsequence of "cabac" because we can delete the last "ac".
 * The answer provided is the shortest such string that satisfies these properties.
 * 
 *  
 * Note:
 * <ol>
 * 	1 <= str1.length, str2.length <= 1000
 * 	str1 and str2 consist of lowercase English letters.
 * </ol>
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-common-supersequence/
// discuss: https://leetcode.com/problems/shortest-common-supersequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // TODO: This solution will time out at some large test case.
    // May consider to sovlve by finding the longest common subsequence first, then the shorted common supersequence. 
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let str1_chars : Vec<char> = str1.chars().collect();
        let str1_len : usize = str1_chars.len();
        let str2_chars : Vec<char> = str2.chars().collect();
        let str2_len : usize = str2_chars.len();

        let mut result = vec![vec!["".to_owned(); str2_len+1];str1_len+1];
        for i in 0..=str1_len {
            result[i][0] = str1_chars.iter().take(i).cloned().collect();
        }
        for j in 0..=str2_len {
            result[0][j] = str2_chars.iter().take(j).cloned().collect();
        }

        for i in 1..=str1_len {
            for j in 1..=str2_len {
                let str1_char : char = str1_chars[i-1];
                let str2_char : char = str2_chars[j-1];
                if str1_char == str2_char {
                    let mut from_diag : String = result[i-1][j-1].clone();
                    from_diag.push(str1_char);
                    result[i][j] = from_diag;
                } else if result[i][j-1].len() < result[i-1][j].len(){
                    let mut from_left : String = result[i][j-1].clone();
                    from_left.push(str2_char);
                    result[i][j] = from_left;
                } else {
                    let mut from_up : String = result[i-1][j].clone();
                    from_up.push(str1_char);
                    result[i][j] = from_up;
                }

                // let mut from_diag : String = result[i-1][j-1].clone();
                // if str1_char == str2_char {
                //     from_diag.push(str1_char);
                // } else {
                //     from_diag.push(str1_char);
                //     from_diag.push(str2_char);
                // }

                // let mut from_left : String = result[i][j-1].clone();
                // from_left.push(str2_char);

                // let mut from_up : String = result[i-1][j].clone();
                // from_up.push(str1_char);

                // result[i][j] = vec![from_diag, from_left, from_up].into_iter().min_by(|a,b|{a.len().cmp(&b.len())}).unwrap();
            }
        }
        result[str1_len][str2_len].clone()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1092() {
    }
}
