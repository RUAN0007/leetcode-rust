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
    pub fn backtrack_helper<P,E,R,B>(result : &mut Vec<Vec<R>>, tmp : &mut Vec<R>, elements : &Vec<E>, predicate: P, parse: B, start : usize, no_dup : bool, element_reusable : bool) where P:Fn(&Vec<R>)->(bool, bool) + Copy, B:Fn(&Vec<E>,usize,usize)->Option<R> + Copy, R:Clone +Eq + std::fmt::Debug, E:std::fmt::Debug{
        // is_sorted() is only supported in nightly-built rust
        // if no_dup && !elements.is_sorted() {
        //     panic!("Elements must be presorted to deduplicate.");
        // }
        if no_dup && element_reusable {
            panic!("element_reusable and no_dup can NOT be both on. ");
        }
        let (valid , early_stop) = predicate(tmp);
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



    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result : Vec<Vec<String>> = vec![];
        let mut tmp : Vec<String> = vec![];
        let element_reusable = false;
        let no_dup = false;

        let predicate = |tmp : &Vec<String>|{
            let early_stop = false;
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
        result
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
            vec![ vec_string!["a", "a", "b"],vec_string!["aa", "b"],]
        );
        assert_eq!(
            Solution::partition("aaa".to_owned()),
            vec![
                vec_string!["a", "a", "a"],
                vec_string!["a", "aa"],
                vec_string!["aa", "a"],
                vec_string!["aaa"],
            ]
        );
    }
}
