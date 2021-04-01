/**
 * [451] Sort Characters By Frequency
 *
 * Given a string, sort it in decreasing order based on the frequency of characters.
 * 
 * Example 1:
 * 
 * Input:
 * "tree"
 * 
 * Output:
 * "eert"
 * 
 * Explanation:
 * 'e' appears twice while 'r' and 't' both appear once.
 * So 'e' must appear before both 'r' and 't'. Therefore "eetr" is also a valid answer.
 * 
 * 
 * 
 * Example 2:
 * 
 * Input:
 * "cccaaa"
 * 
 * Output:
 * "cccaaa"
 * 
 * Explanation:
 * Both 'c' and 'a' appear three times, so "aaaccc" is also a valid answer.
 * Note that "cacaca" is incorrect, as the same characters must be together.
 * 
 * 
 * 
 * Example 3:
 * 
 * Input:
 * "Aabb"
 * 
 * Output:
 * "bbAa"
 * 
 * Explanation:
 * "bbaA" is also a valid answer, but "Aabb" is incorrect.
 * Note that 'A' and 'a' are treated as two different characters.
 * 
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-characters-by-frequency/
// discuss: https://leetcode.com/problems/sort-characters-by-frequency/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut frequency = HashMap::new();
        s.chars().for_each(|c| {
            if let Some(f) = frequency.get_mut(&c) {
                *f += 1;
            } else {
                frequency.insert(c, 1);
            }
        });

        let mut sorted_char = vec![];
        for (&c, &count) in frequency.iter() {
            sorted_char.push((count, c));
        }
        sorted_char.sort();
        sorted_char.reverse();
        let mut result = String::from("");
        for (count, c) in sorted_char {
            for i in 0..count as usize {
                result.push(c);
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
    fn test_451() {
    }
}
