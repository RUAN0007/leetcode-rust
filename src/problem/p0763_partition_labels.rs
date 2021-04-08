/**
 * [763] Partition Labels
 *
 * A string S of lowercase English letters is given. We want to partition this string into as many parts as possible so that each letter appears in at most one part, and return a list of integers representing the size of these parts.
 *  
 * Example 1:
 * 
 * Input: S = "ababcbacadefegdehijhklij"
 * Output: [9,7,8]
 * Explanation:
 * The partition is "ababcbaca", "defegde", "hijhklij".
 * This is a partition so that each letter appears in at most one part.
 * A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it splits S into less parts.
 * 
 *  
 * Note:
 * 
 * 	S will have length in range [1, 500].
 * 	S will consist of lowercase English letters ('a' to 'z') only.
 * 
 *  
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-labels/
// discuss: https://leetcode.com/problems/partition-labels/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn char2idx(c : char) ->usize {
        let base_idx = 'a' as u8 as usize;
        let c_idx = c as u8 as usize;
        c_idx - base_idx
    }

    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut result = vec![];

        let mut char_positions= vec![vec![];26];
        for (i,c) in s.chars().enumerate() {
            char_positions[Self::char2idx(c)].push(i);
        }

        let mut i = 0;
        // println!("char_positions: {:?}", char_positions);
        let ss : Vec<char> = s.chars().collect();
        while i < s.len() {
            let mut l = 0;
            let mut cur_max = i;
            while i <= cur_max {
                let cur_char = Self::char2idx(ss[i]);
                cur_max = std::cmp::max(cur_max, *char_positions[cur_char].iter().max().unwrap());
                // println!("i={}, cur_char={}, cur_max={}", i, cur_char, cur_max);
                i+=1;
                l+=1;
            }
            result.push(l);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_763() {
        assert_eq!(Solution::partition_labels("ababcbacadefegdehijhklij".to_owned()), vec![9,7,8]);
    }
}
