/**
 * [39] Combination Sum
 *
 * Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target. You may return the combinations in any order.
 * The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the frequency of at least one of the chosen numbers is different.
 * It is guaranteed that the number of unique combinations that sum up to target is less than 150 combinations for the given input.
 *  
 * Example 1:
 * 
 * Input: candidates = [2,3,6,7], target = 7
 * Output: [[2,2,3],[7]]
 * Explanation:
 * 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
 * 7 is a candidate, and 7 = 7.
 * These are the only two combinations.
 * 
 * Example 2:
 * 
 * Input: candidates = [2,3,5], target = 8
 * Output: [[2,2,2,2],[2,3,3],[3,5]]
 * 
 * Example 3:
 * 
 * Input: candidates = [2], target = 1
 * Output: []
 * 
 * Example 4:
 * 
 * Input: candidates = [1], target = 1
 * Output: [[1]]
 * 
 * Example 5:
 * 
 * Input: candidates = [1], target = 2
 * Output: [[1,1]]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= candidates.length <= 30
 * 	1 <= candidates[i] <= 200
 * 	All elements of candidates are distinct.
 * 	1 <= target <= 500
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/combination-sum/
// discuss: https://leetcode.com/problems/combination-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn recursive_helper<P>(result : &mut Vec<Vec<i32>>, tmp : &mut Vec<i32>, elements : &Vec<i32>, predicate: P, start : usize, no_dup : bool, element_reusable : bool) where P:Fn(&Vec<i32>)->(bool, bool) + Copy {
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
                    Self::recursive_helper(result, tmp, elements, predicate, next_start, no_dup, element_reusable);
                    tmp.pop();
                }
            }
        }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result : Vec<Vec<i32>> = vec![];
        let mut tmp : Vec<i32> = vec![];
        let element_reusable = true;
        let no_dup = false;

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

        Self::recursive_helper(&mut result, &mut tmp, &candidates, predicate, 0, no_dup, element_reusable);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_39() {
        assert_eq!(
            Solution::combination_sum(vec![1], 7),
            vec![vec![1, 1, 1, 1, 1, 1, 1]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
    }
}
