/**
 * [132] Palindrome Partitioning II
 *
 * Given a string s, partition s such that every substring of the partition is a palindrome.
 * Return the minimum cuts needed for a palindrome partitioning of s.
 *  
 * Example 1:
 * 
 * Input: s = "aab"
 * Output: 1
 * Explanation: The palindrome partitioning ["aa","b"] could be produced using 1 cut.
 * 
 * Example 2:
 * 
 * Input: s = "a"
 * Output: 0
 * 
 * Example 3:
 * 
 * Input: s = "ab"
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 2000
 * 	s consists of lower-case English letters only.
 * 
 */
use std::collections::HashSet;
pub struct Solution {
}

// problem: https://leetcode.com/problems/palindrome-partitioning-ii/
// discuss: https://leetcode.com/problems/palindrome-partitioning-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn backtrack_helper<P,E,R,B>(result : &mut Vec<Vec<R>>, tmp : &mut Vec<R>, elements : &Vec<E>, predicate: P, parse: B, start : usize, no_dup : bool, element_reusable : bool) where P:Fn(&Vec<Vec<R>>, &Vec<R>)->(bool, bool) + Copy, B:Fn(&Vec<E>,usize,usize)->Option<R> + Copy, R:Clone +Eq + std::fmt::Debug, E:std::fmt::Debug{
        // is_sorted() is only supported in nightly-built rust
        // if no_dup && !elements.is_sorted() {
        //     panic!("Elements must be presorted to deduplicate.");
        // }
        if no_dup && element_reusable {
            panic!("element_reusable and no_dup can NOT be both on. ");
        }
        let (valid , early_stop) = predicate(result, tmp);
        if valid { result.push(tmp.clone()); }
        if early_stop {return}

        let n : usize = elements.len();
        let mut last_parsed : Option<R> = None;
        let mut start_parse_idx : usize = start;
        for i in start..n {
            let parsed : Option<R> = parse(elements, start, i);
            // println!("elements={:?}, start_idx={}, end={}, parsed={:?}", elements, start, i, parsed);
            if parsed.is_none() {
                continue;
            }
            let parsed : R = parsed.unwrap();

            let mut backtrack = true;
            if no_dup && last_parsed.is_some() && last_parsed.as_ref().unwrap().eq(&parsed) { 
                backtrack = false;
            }

            if backtrack {
                tmp.push(parsed.clone());
                let next_start = if element_reusable { start_parse_idx} else { i+1 };
                Self::backtrack_helper(result, tmp, elements, predicate, parse, next_start, no_dup, element_reusable);
                tmp.pop();

            }
            last_parsed = Some(parsed.clone());
            start_parse_idx = i + 1;
        }
    }

    // May timeout
    pub fn min_cut_recursive(s: String) -> i32 {
        let mut result : Vec<Vec<String>> = vec![];
        let mut tmp : Vec<String> = vec![];
        let element_reusable = false;
        let no_dup = false;

        let predicate = |result: &Vec<Vec<String>>, tmp : &Vec<String>|{
            let min_cut = result.iter().map(|r|{r.len()}).min();
            let cur_cut = tmp.len();
            let mut early_stop = false;
            if min_cut.is_some() && min_cut.unwrap() <=cur_cut {
                early_stop = true;
            }

            let mut l = 0usize;
            for t in tmp.iter() {
                l+=t.len();
            }
            (l==s.len(), early_stop)
        };

        let parse = |elements : &Vec<char>, start_idx : usize, end_idx : usize|{
            // end_idx is inclusive
            let chars : Vec<char> = elements.iter().skip(start_idx).take(end_idx + 1 - start_idx).cloned().collect();
            let n = chars.len();
            for i in 0..n/2 {
                if chars[i] != chars[n-1-i] {return None}
            }
            let parsed : String = chars.into_iter().collect();
            Some(parsed)
        };

        let elements : Vec<char> = s.chars().collect();
        Self::backtrack_helper(&mut result, &mut tmp, &elements, predicate, parse, 0, no_dup, element_reusable);
        (result.iter().map(|r|{r.len()}).min().unwrap() - 1) as i32
    }

    // with DP
    pub fn min_cut(s: String) -> i32 {
        let n = s.len() as i32;
        let mut min_cuts = vec![n as usize;n as usize]; 
        let mut is_palindrome = HashSet::new();
        let s_chars : Vec<char> = s.chars().collect();
        for end in 0..n {//inclusive
            for start in 0..=end {
                if s_chars[start as usize] == s_chars[end as usize] && (start+1>end-1||is_palindrome.contains(&(start+1,end-1))) {
                    is_palindrome.insert((start, end));
                    if start == 0 {
                        min_cuts[end as usize] = 0;
                    } else {
                        min_cuts[end as usize] = std::cmp::min(min_cuts[end as usize], 1+min_cuts[start as usize-1]);
                    }
                }
            }
        }
        min_cuts[n as usize -1] as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_132() {
        assert_eq!(Solution::min_cut("aab".to_owned()), 1);
        assert_eq!(Solution::min_cut("aaa".to_owned()), 0);
        assert_eq!(Solution::min_cut("aabb".to_owned()), 1);
    }
}
