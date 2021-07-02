/**
 * [395] Longest Substring with At Least K Repeating Characters
 *
 * Given a string s and an integer k, return the length of the longest substring of s such that the frequency of each character in this substring is greater than or equal to k.
 *  
 * Example 1:
 * 
 * Input: s = "aaabb", k = 3
 * Output: 3
 * Explanation: The longest substring is "aaa", as 'a' is repeated 3 times.
 * 
 * Example 2:
 * 
 * Input: s = "ababbc", k = 2
 * Output: 5
 * Explanation: The longest substring is "ababb", as 'a' is repeated 2 times and 'b' is repeated 3 times.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^4
 * 	s consists of only lowercase English letters.
 * 	1 <= k <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/
// discuss: https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn helper(s : &Vec<char>, mut start : usize, end : usize, k : usize) -> usize {
        // println!("start={}, end={}", start, end);
        if end <= start {return 0}
        let len : usize = end - start;
        let mut freq : [usize;26]  = [0;26];
        for i in start..end {
            let char_pos : usize = ((s[i] as u8) - ('a' as u8)) as usize;
            freq[char_pos]+=1;
        }
        let mut i : usize = start;
        let mut next_start : usize = start;
        let mut r : usize = 0;
        let mut full = true;
        while i < end {
            let char_pos : usize = ((s[i] as u8) - ('a' as u8)) as usize;
            if freq[char_pos] < k {
                r = r.max(Self::helper(s, next_start, i, k));
                next_start = i + 1;
                full = false;
            }
            i+=1;
        }
        if next_start != start {
            r = r.max(Self::helper(s, next_start, i, k));
        }
        if full {len} else {r}
    }

    pub fn longest_substring(s: String, k: i32) -> i32 {
        let s : Vec<char> = s.chars().collect();
        Self::helper(&s, 0, s.len(), k as usize) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_395() {
        assert_eq!(Solution::longest_substring("aaabb".to_owned(), 3), 3);
    }
}
