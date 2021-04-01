/**
 * [187] Repeated DNA Sequences
 *
 * The DNA sequence is composed of a series of nucleotides abbreviated as 'A', 'C', 'G', and 'T'.
 * 
 * 	For example, "ACGAATTCCG" is a DNA sequence.
 * 
 * When studying DNA, it is useful to identify repeated sequences within the DNA.
 * Given a string s that represents a DNA sequence, return all the 10-letter-long sequences (substrings) that occur more than once in a DNA molecule. You may return the answer in any order.
 *  
 * Example 1:
 * Input: s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
 * Output: ["AAAAACCCCC","CCCCCAAAAA"]
 * Example 2:
 * Input: s = "AAAAAAAAAAAAA"
 * Output: ["AAAAAAAAAA"]
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^5
 * 	s[i] is either 'A', 'C', 'G', or 'T'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/repeated-dna-sequences/
// discuss: https://leetcode.com/problems/repeated-dna-sequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::{collections::HashSet};
 
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut dup = HashSet::new();
        let mut dup_result = vec![];
        if s.len() <= 10 {
            return vec![]
        }

        for i in 0..=s.len()-10 {
            let sub = String::from(&s.as_str()[i..i+10]);
            if dup.contains(&sub) {
                // println!("substr={}, already exists...", sub);
                dup_result.push(sub);
            } else {
                // println!("substr={}, not exists...", sub);
                dup.insert(sub);
            }
        }
        dup_result.sort();
        let mut result = vec![];
        for i in 0..dup_result.len() {
            if i == 0 || dup_result[i-1] != dup_result[i] {
                result.push(dup_result[i].clone());
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_187() {
        assert_eq!(
            Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_owned()),
            vec_string!["AAAAACCCCC", "CCCCCAAAAA"]
        );
        assert_eq!(
            Solution::find_repeated_dna_sequences("GAGAGAGAGAGA".to_owned()),
            vec_string!["GAGAGAGAGA"]
        );
    }
}
