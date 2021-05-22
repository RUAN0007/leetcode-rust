/**
 * [87] Scramble String
 *
 * We can scramble a string s to get a string t using the following algorithm:
 * <ol>
 * 	If the length of the string is 1, stop.
 * 	If the length of the string is > 1, do the following:
 * 	
 * 		Split the string into two non-empty substrings at a random index, i.e., if the string is s, divide it to x and y where s = x + y.
 * 		Randomly decide to swap the two substrings or to keep them in the same order. i.e., after this step, s may become s = x + y or s = y + x.
 * 		Apply step 1 recursively on each of the two substrings x and y.
 * 	
 * 	
 * </ol>
 * Given two strings s1 and s2 of the same length, return true if s2 is a scrambled string of s1, otherwise, return false.
 *  
 * Example 1:
 * 
 * Input: s1 = "great", s2 = "rgeat"
 * Output: true
 * Explanation: One possible scenario applied on s1 is:
 * "great" --> "gr/eat" // divide at random index.
 * "gr/eat" --> "gr/eat" // random decision is not to swap the two substrings and keep them in order.
 * "gr/eat" --> "g/r / e/at" // apply the same algorithm recursively on both substrings. divide at ranom index each of them.
 * "g/r / e/at" --> "r/g / e/at" // random decision was to swap the first substring and to keep the second substring in the same order.
 * "r/g / e/at" --> "r/g / e/ a/t" // again apply the algorithm recursively, divide "at" to "a/t".
 * "r/g / e/ a/t" --> "r/g / e/ a/t" // random decision is to keep both substrings in the same order.
 * The algorithm stops now and the result string is "rgeat" which is s2.
 * As there is one possible scenario that led s1 to be scrambled to s2, we return true.
 * 
 * Example 2:
 * 
 * Input: s1 = "abcde", s2 = "caebd"
 * Output: false
 * 
 * Example 3:
 * 
 * Input: s1 = "a", s2 = "a"
 * Output: true
 * 
 *  
 * Constraints:
 * 
 * 	s1.length == s2.length
 * 	1 <= s1.length <= 30
 * 	s1 and s2 consist of lower-case English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/scramble-string/
// discuss: https://leetcode.com/problems/scramble-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn is_scramble_topdown(s1: &Vec<char>, start1 : usize, end1 : usize, 
        s2: &Vec<char>, start2 : usize, end2 : usize, cache : &mut HashMap<(usize,usize,usize), bool>, level : usize) -> bool {
        assert_eq!(end1 - start1, end2 - start2);
        let pad : String = (0..level).map(|x|{"  "}).collect();
        let l : usize = end1 - start1;
        if l == 1 { return s1[start1] == s2[start2]; }
        if let Some(&cached) = cache.get(&(start1, start2, l)) { 
            return cached; 
        }

        println!("{}start1={}, end1={}, start2={}, end2={}", pad, start1, end1, start2, end2);
        let mut result : bool = false;
        for i in 1..l {
            let mid1 : usize = start1 + i;
            let mid2 : usize = start2 + i;

            println!("{}i={},mid1={}, mid2={}",pad, i, mid1, mid2);
            if Self::is_scramble_topdown(s1, start1, mid1, s2, start2, mid2, cache, level+1) && 
                Self::is_scramble_topdown(s1, mid1, end1, s2, mid2, end2, cache, level + 1) {
                    result = true;
                    break;
            }

            let mid1 : usize = start1 + i;
            let mid2 : usize = end2 - i;

            println!("{}i={},mid1={}, mid2={}",pad, i, mid1, mid2);
            if Self::is_scramble_topdown(s1, start1, mid1, s2, mid2, end2, cache, level + 1) && 
                Self::is_scramble_topdown(s1, mid1, end1, s2, start2, mid2, cache, level+1) {
                    result = true;
                    break;
            }

        }
        cache.insert((start1, start2, l), result);
        println!("{}start1={}, end1={}, start2={}, end2={}, result={}", pad, start1, end1, start2, end2, result);
        result
    }

    pub fn is_scramble(s1: String, s2: String) -> bool {
        let s1 : Vec<char> = s1.chars().collect();
        let s2 : Vec<char> = s2.chars().collect();
        let mut cache : HashMap<(usize, usize, usize), bool> = HashMap::new();
        Self::is_scramble_topdown(&s1, 0, s1.len(), &s2, 0, s2.len(), &mut cache, 0)
    }

    pub fn is_scramble_bottomup(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {return false;}
        let l : usize = s1.len();
        // scrambled[k][i][j] denote whether s1[i..i+k] can be scrambled from s2[i..i+k]. 
        let mut scrambled : Vec<Vec<Vec<bool>>> = vec![vec![vec![false; l]; l]; l + 1];
        let s1 : Vec<char> = s1.chars().collect();
        let s2 : Vec<char> = s2.chars().collect();
        for k in 1..=l {
            for i in 0..=(l-k) {
                for j in 0..=(l-k) {
                    if k == 1 {
                        scrambled[k][i][j] = s1[i] == s2[j];
                    } else {
                        for m in 1..=k-1 {
                            if scrambled[m][i][j] && scrambled[k-m][i+m][j+m] {
                                scrambled[k][i][j] = true;
                                break;
                            }

                            if scrambled[m][i][j+k-m] && scrambled[k-m][i+m][j] {
                                scrambled[k][i][j] = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
        
        scrambled[l][0][0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_87() {
        // assert!(Solution::is_scramble("great".to_owned(), "rgeat".to_owned()));
        // assert!(!Solution::is_scramble("abcde".to_owned(), "caebd".to_owned()));

        assert!(Solution::is_scramble("abcd".to_owned(), "cdab".to_owned()));
    }
}
