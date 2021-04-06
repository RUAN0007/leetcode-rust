/**
 * [318] Maximum Product of Word Lengths
 *
 * Given a string array words, return the maximum value of length(word[i]) * length(word[j]) where the two words do not share common letters. If no such two words exist, return 0.
 *  
 * Example 1:
 * 
 * Input: words = ["abcw","baz","foo","bar","xtfn","abcdef"]
 * Output: 16
 * Explanation: The two words can be "abcw", "xtfn".
 * 
 * Example 2:
 * 
 * Input: words = ["a","ab","abc","d","cd","bcd","abcd"]
 * Output: 4
 * Explanation: The two words can be "ab", "cd".
 * 
 * Example 3:
 * 
 * Input: words = ["a","aa","aaa","aaaa"]
 * Output: 0
 * Explanation: No such pair of words.
 * 
 *  
 * Constraints:
 * 
 * 	2 <= words.length <= 1000
 * 	1 <= words[i].length <= 1000
 * 	words[i] consists only of lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-product-of-word-lengths/
// discuss: https://leetcode.com/problems/maximum-product-of-word-lengths/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here


impl Solution { pub fn max_product(words: Vec<String>) -> i32 {
        let mut word_int = vec![0; words.len()];
        let mut result = 0;
        for (i, word) in words.iter().enumerate() {
            // word[i] encodes words[i] in the below format:
            // if char x exists, the x-'a' digit is set. 
            word_int[i] =
                word.chars().fold(0, |acc, c|{acc|(1 << (c as usize - 'a' as usize))});
            for j in 0..i {
                if word_int[i] & word_int[j] == 0 {
                    result = std::cmp::max(result, words[i].len() * words[j].len());
                }
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
    fn test_318() {
    }
}
