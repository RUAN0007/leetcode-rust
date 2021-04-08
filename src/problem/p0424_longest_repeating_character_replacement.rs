/**
 * [424] Longest Repeating Character Replacement
 *
 * Given a string s that consists of only uppercase English letters, you can perform at most k operations on that string.
 * 
 * In one operation, you can choose any character of the string and change it to any other uppercase English character.
 * 
 * Find the length of the longest sub-string containing all repeating letters you can get after performing the above operations.
 * 
 * Note:<br />
 * Both the string's length and k will not exceed 10^4.
 * 
 * Example 1:
 * 
 * 
 * Input:
 * s = "ABAB", k = 2
 * 
 * Output:
 * 4
 * 
 * Explanation:
 * Replace the two 'A's with two 'B's or vice versa.
 * 
 * 
 *  
 * 
 * Example 2:
 * 
 * 
 * Input:
 * s = "AABABBA", k = 1
 * 
 * Output:
 * 4
 * 
 * Explanation:
 * Replace the one 'A' in the middle with 'B' and form "AABBBBA".
 * The substring "BBBB" has the longest repeating letters, which is 4.
 * 
 * 
 *  
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-repeating-character-replacement/
// discuss: https://leetcode.com/problems/longest-repeating-character-replacement/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
       let ss : Vec<usize> = s.chars().map(|x|{x as usize}).collect();
       let k = k as usize;
       let mut start = 0usize;
       let mut end = 0usize;
       // in the considered substr. 
       let mut char_counts = vec![0; 128];
       let mut cur_max = 0usize;
       while end < s.len() {
          let end_char = ss[end];
          char_counts[end_char]+=1;
        //   println!("END ss[{}]={}", end, end_char as u8 as char);
        //   println!("char_count={}, max_char_count={}", char_counts.iter().sum::<usize>(), *char_counts.iter().max().unwrap());
          while  char_counts.iter().sum::<usize>() - *char_counts.iter().max().unwrap() > k {
            let start_char = ss[start];
            // println!("  START ss[{}]={}", start, start_char as u8 as char);
            char_counts[start_char]-=1;
            // println!("  char_count={}, max_char_count{}", char_counts.iter().sum::<usize>(), *char_counts.iter().max().unwrap());
            start+=1;

          }
          cur_max = std::cmp::max(cur_max, char_counts.iter().sum());
          end+=1;

       }
       cur_max as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_424() {
        assert_eq!(Solution::character_replacement("ABAB".to_owned(), 2), 4);
        assert_eq!(Solution::character_replacement("AABABBA".to_owned(), 1), 4);
    }
}
