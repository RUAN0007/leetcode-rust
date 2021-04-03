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
        let mut char_count = HashMap::new();
        s.chars().for_each(|c|{
            if let Some(c_count) = char_count.get_mut(&c) {
                *c_count += 1;
            } else {
                char_count.insert(c, 1);
            }
        });


        let mut max_fqc = 0;
        let mut fqt_char = 'A';
        for (&c, &f) in &char_count {
            if max_fqc < f {
                max_fqc = f;
                fqt_char = c;
            }
        }
        println!("char_count = {:?}", char_count);
        println!("max_fqc = {:?}, fqt_char = {:?}", max_fqc, fqt_char);
        // let max_frequency = *char_count.values().max().unwrap();
        let n = s.len();
        if max_fqc <= (n+1) / 2 {
            let mut idx_map : Vec<usize> = (0..(s.len()+1)/2).map(|idx|{2*idx}).collect();
            idx_map.append(&mut (0..(s.len())/2).map(|idx|1+{2*idx}).collect::<Vec<usize>>());
            println!("idx_map:{:?}", idx_map);

            let mut i = 0usize;
            let mut result = vec!['a';s.len()];

            let start = i;
            while i < start + max_fqc {
                result[idx_map[i]] = fqt_char;
                i += 1;
            }

            char_count.remove(&fqt_char);
            for (c, f) in char_count {
                let start = i;
                while i < start + f {
                    println!("\ti={}, f={}, start={}, idx_map={:?}",i, f, start, idx_map);
                    result[idx_map[i]] = c;
                    i += 1;
                }
            }
            result.iter().collect()

        } else {
            "".to_owned()
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_767() {
        assert_eq!(Solution::reorganize_string("aab".to_owned()),"aba");
        assert_eq!(Solution::reorganize_string("abbabbaaab".to_owned()),"ababababab");
    }
}
