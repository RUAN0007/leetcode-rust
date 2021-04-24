/**
 * [767] Reorganize String
 *
 * Given a string S, check if the letters can be rearranged so that two characters that are adjacent to each other are not the same.
 * 
 * If possible, output any possible result.  If not possible, return the empty string.
 * 
 * Example 1:
 * 
 * 
 * Input: S = "aab"
 * Output: "aba"
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: S = "aaab"
 * Output: ""
 * 
 * 
 * Note:
 * 
 * 
 * 	S will consist of lowercase letters and have length in range [1, 500].
 * 
 * 
 *  
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reorganize-string/
// discuss: https://leetcode.com/problems/reorganize-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut char_counts : Vec<usize> = vec![0;26];
        for c in s.chars() {
            char_counts[c as u8 as usize - 'a' as u8 as usize] +=1;
        }
        let n = s.len();
        let mut idx_map : Vec<usize> = (0..(n+1)/2).map(|x|{2*x}).collect();
        idx_map.extend((0..(n/2)).map(|x|{2*x+1}).collect::<Vec<usize>>());

        let mut i : usize = 0;
        let mut result : Vec<char> = vec!['a';n];
        while i < n {
            let mut cur_max_char = 0;
            let mut cur_max_count = 0;
            for c in 0..26 {
                if  cur_max_count < char_counts[c] {
                    cur_max_char = c;
                    cur_max_count = char_counts[c];
                }
            }
            if cur_max_count > (n+1)/2 {return "".to_owned();};
            let end = i + cur_max_count;
            // println!("i={}, n={}, end={}, cur_max_char={}",i,n, end, cur_max_char);
            while i < end {
                result[idx_map[i]] = ('a' as u8 + cur_max_char as u8) as char;
                i+=1;
            }
            char_counts[cur_max_char] = 0;
        }
        result.iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_767() {
        assert_eq!(Solution::reorganize_string("abbabbaaab".to_owned()),"ababababab");
    }
}
