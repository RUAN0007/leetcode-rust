/**
 * [216] Combination Sum III
 *
 * Find all valid combinations of k numbers that sum up to n such that the following conditions are true:
 * 
 * 	Only numbers 1 through 9 are used.
 * 	Each number is used at most once.
 * 
 * Return a list of all possible valid combinations. The list must not contain the same combination twice, and the combinations may be returned in any order.
 *  
 * Example 1:
 * 
 * Input: k = 3, n = 7
 * Output: [[1,2,4]]
 * Explanation:
 * 1 + 2 + 4 = 7
 * There are no other valid combinations.
 * Example 2:
 * 
 * Input: k = 3, n = 9
 * Output: [[1,2,6],[1,3,5],[2,3,4]]
 * Explanation:
 * 1 + 2 + 6 = 9
 * 1 + 3 + 5 = 9
 * 2 + 3 + 4 = 9
 * There are no other valid combinations.
 * 
 * Example 3:
 * 
 * Input: k = 4, n = 1
 * Output: []
 * Explanation: There are no valid combinations. [1,2,1] is not valid because 1 is used twice.
 * 
 * Example 4:
 * 
 * Input: k = 3, n = 2
 * Output: []
 * Explanation: There are no valid combinations.
 * 
 * Example 5:
 * 
 * Input: k = 9, n = 45
 * Output: [[1,2,3,4,5,6,7,8,9]]
 * Explanation:
 * 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 = 45
 * ​​​​​​​There are no other valid combinations.
 * 
 *  
 * Constraints:
 * 
 * 	2 <= k <= 9
 * 	1 <= n <= 60
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/combination-sum-iii/
// discuss: https://leetcode.com/problems/combination-sum-iii/discuss/?currentPage=1&orderBy=most_votes&query=

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


    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut result : Vec<Vec<i32>> = vec![];
        let mut tmp : Vec<i32> = vec![];
        // let predicate = ||{}
        let elements : Vec<i32> = (1..=9).collect();
        let parse = |elements : &Vec<i32>, start_idx : usize, cur_idx : usize|{Some(elements[cur_idx])};
        let predicate = |result : &Vec<Vec<i32>>, tmp : &Vec<i32>| {
            let mut valid = false;
            let mut early_stop = false;
            if tmp.iter().sum::<i32>() == n && tmp.len() == k {
                valid = true;
            }
            if tmp.len() >= k {
                early_stop = true;
            }
            if tmp.iter().sum::<i32>() >= n {
                early_stop = true;
            }

            (valid, early_stop)
        };
        Self::backtrack_helper(&mut result, &mut tmp, &elements, predicate, parse, 0, false, false);

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_216() {
        assert_eq!(
            Solution::combination_sum3(3, 9),
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
        );
    }
}
