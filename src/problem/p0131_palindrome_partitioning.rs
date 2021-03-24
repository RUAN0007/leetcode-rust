/**
 * [131] Palindrome Partitioning
 *
 * Given a string s, partition s such that every substring of the partition is a palindrome. Return all possible palindrome partitioning of s.
 * A palindrome string is a string that reads the same backward as forward.
 *  
 * Example 1:
 * Input: s = "aab"
 * Output: [["a","a","b"],["aa","b"]]
 * Example 2:
 * Input: s = "a"
 * Output: [["a"]]
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 16
 * 	s contains only lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-partitioning/
// discuss: https://leetcode.com/problems/palindrome-partitioning/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_palindrome (s: &String) -> bool {
        let l = s.len();
        for i in 0..l/2 {
            if s.chars().nth(i).unwrap() != s.chars().nth(l-i-1).unwrap() {
                return false;
            }
        }
        true
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        // The corresponding partition result for each i, where i is the starting char. 
        let mut all_result : Vec<Vec<Vec<String>>> = vec![vec![vec![]];s.len()+1];

        for i in (0..s.len()).rev() {
            let mut this_result : Vec<Vec<String>> = vec![];

            for j in i..s.len() {

                let mut sub_str = String::from(&s.as_str()[i..j+1]);

                // sub_str.push(s.chars().nth(j).unwrap());
                println!("\tj = {}, sub_str = {}", j, sub_str);
                if Self::is_palindrome(&sub_str) {
                    println!("\tYES Palindrome sub_str = {}",  sub_str);

                    let prev_result = all_result[j+1].clone();
                    let mut this_pal_result = vec![];
                    for mut r in prev_result {
                        let mut n = vec![sub_str.clone()];
                        n.append(&mut r);
                        this_pal_result.push(n);
                    }
                    this_result.append(&mut this_pal_result);
                }
            }  // for j
            println!("i = {}, this_result = {:?}", i, this_result);
            all_result[i] = this_result;
        }  // for i

        all_result[0].clone()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_131() {
        assert_eq!(
            Solution::partition("aab".to_owned()),
            vec![vec_string!["aa", "b"], vec_string!["a", "a", "b"],]
        );
        assert_eq!(
            Solution::partition("aaa".to_owned()),
            vec![
                vec_string!["aaa"],
                vec_string!["a", "aa"],
                vec_string!["aa", "a"],
                vec_string!["a", "a", "a"],
            ]
        );
    }
}
