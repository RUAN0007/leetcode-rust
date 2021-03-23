/**
 * [91] Decode Ways
 *
 * A message containing letters from A-Z can be encoded into numbers using the following mapping:
 * 
 * 'A' -> "1"
 * 'B' -> "2"
 * ...
 * 'Z' -> "26"
 * 
 * To decode an encoded message, all the digits must be grouped then mapped back into letters using the reverse of the mapping above (there may be multiple ways). For example, "11106" can be mapped into:
 * 
 * 	"AAJF" with the grouping (1 1 10 6)
 * 	"KJF" with the grouping (11 10 6)
 * 
 * Note that the grouping (1 11 06) is invalid because "06" cannot be mapped into 'F' since "6" is different from "06".
 * Given a string s containing only digits, return the number of ways to decode it.
 * The answer is guaranteed to fit in a 32-bit integer.
 *  
 * Example 1:
 * 
 * Input: s = "12"
 * Output: 2
 * Explanation: "12" could be decoded as "AB" (1 2) or "L" (12).
 * 
 * Example 2:
 * 
 * Input: s = "226"
 * Output: 3
 * Explanation: "226" could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
 * 
 * Example 3:
 * 
 * Input: s = "0"
 * Output: 0
 * Explanation: There is no character that is mapped to a number starting with 0.
 * The only valid mappings with 0 are 'J' -> "10" and 'T' -> "20", neither of which start with 0.
 * Hence, there are no valid ways to decode this since all digits need to be mapped.
 * 
 * Example 4:
 * 
 * Input: s = "06"
 * Output: 0
 * Explanation: "06" cannot be mapped to "F" because of the leading zero ("6" is different from "06").
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 100
 * 	s contains only digits and may contain leading zero(s).
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/decode-ways/
// discuss: https://leetcode.com/problems/decode-ways/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut result : Vec<i32> = vec![0;s.len()+1];
        result[s.len()] = 1;

        for i in (0..s.len()).rev() {
           
           match((s.chars().nth(i), s.chars().nth(i+1))) {
            (Some('0'), _) => {result[i] = 0},
            (Some(_), None) => {
                result[i] = 1
            },

            (Some('3'), Some(_)) |  (Some('4'), Some(_)) |
            (Some('5'), Some(_)) |  (Some('6'), Some(_)) |
            (Some('7'), Some(_)) |  (Some('8'), Some(_)) | 
            (Some('9'), Some(_)) |
            (Some('2'), Some('7')) |
            (Some('2'), Some('8')) | (Some('2'), Some('9')) 
                => {result[i] = result[i+1]},

            (Some('1'), _) | (Some('2'), Some('0')) |
            (Some('2'), Some('1')) | (Some('2'), Some('2')) |
            (Some('2'), Some('3')) | (Some('2'), Some('4')) |
            (Some('2'), Some('5')) | (Some('2'), Some('6'))
            => {result[i] = result[i+1] + result[i+2]},

            _ => {panic!("Unrecognized case at {}", i)}

           };
        }
        println!("result = {:?}", result);
        result[0]
    }

}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_91() {
        assert_eq!(
        Solution::num_decodings(
            "12".to_owned()
        ), 2);
    }
}
