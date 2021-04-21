/**
 * [40] Combination Sum II
 *
 * Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sum to target.
 * Each number in candidates may only be used once in the combination.
 * Note: The solution set must not contain duplicate combinations.
 *  
 * Example 1:
 * 
 * Input: candidates = [10,1,2,7,6,1,5], target = 8
 * Output: 
 * [
 * [1,1,6],
 * [1,2,5],
 * [1,7],
 * [2,6]
 * ]
 * 
 * Example 2:
 * 
 * Input: candidates = [2,5,2,1,2], target = 5
 * Output: 
 * [
 * [1,2,2],
 * [5]
 * ]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= candidates.length <= 100
 * 	1 <= candidates[i] <= 50
 * 	1 <= target <= 30
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/combination-sum-ii/
// discuss: https://leetcode.com/problems/combination-sum-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn backtrack_helper<P>(result : &mut Vec<Vec<i32>>, tmp : &mut Vec<i32>, elements : &Vec<i32>, predicate: P, start : usize, no_dup : bool, element_reusable : bool) where P:Fn(&Vec<i32>)->(bool, bool) + Copy {
        // is_sorted() is only supported in nightly-built rust
        // if no_dup && !elements.is_sorted() {
        //     panic!("Elements must be presorted to deduplicate.");
        // }
        let (valid , backtrack) = predicate(tmp);
        if valid {
            result.push(tmp.clone());
        }
        if backtrack {
            let n : usize = elements.len();
            for i in start..n {
                let backtrack : bool = if !no_dup {true} else if i==start{true}else if elements[i-1] != elements[i] {true}else{false};

                if backtrack {
                    tmp.push(elements[i]);
                    let next_start = if element_reusable { i } else { i+1 };
                    Self::backtrack_helper(result, tmp, elements, predicate, next_start, no_dup, element_reusable);
                    tmp.pop();
                }
            }
        }
    }

    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result : Vec<Vec<i32>> = vec![];
        let mut tmp : Vec<i32> = vec![];
        let element_reusable = false;
        let no_dup = true;
        candidates.sort();

        let predicate = |tmp : &Vec<i32>|{
            let mut valid = false;
            let mut backtrack = false;
            let sum : i32 = tmp.iter().sum();
            if sum < target {
                backtrack = true;
            } else if sum == target {
                valid = true;
            }
            (valid, backtrack)
        };

        Self::backtrack_helper(&mut result, &mut tmp, &candidates, predicate, 0, no_dup, element_reusable);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_40() {
        assert_eq!(
            Solution::combination_sum2(vec![1, 1, 1, 1, 1, 1, 1], 7),
            vec![vec![1, 1, 1, 1, 1, 1, 1]]
        );
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5]]
        );
    }
}
