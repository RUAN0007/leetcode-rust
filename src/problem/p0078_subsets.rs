/**
 * [78] Subsets
 *
 * Given an integer array nums of unique elements, return all possible subsets (the power set).
 * The solution set must not contain duplicate subsets. Return the solution in any order.
 *  
 * Example 1:
 * 
 * Input: nums = [1,2,3]
 * Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
 * 
 * Example 2:
 * 
 * Input: nums = [0]
 * Output: [[],[0]]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10
 * 	-10 <= nums[i] <= 10
 * 	All the numbers of nums are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/subsets/
// discuss: https://leetcode.com/problems/subsets/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here


impl Solution {
    pub fn recursive_helper<P>(result : &mut Vec<Vec<i32>>, tmp : &mut Vec<i32>, elements : &Vec<i32>, predicate: P, start : usize, no_dup : bool, element_reusable : bool) where P:Fn(&Vec<i32>, start : usize, n : usize)->(bool, bool) + Copy {
        // is_sorted() is only supported in nightly-built rust
        // if no_dup && !elements.is_sorted() {
        //     panic!("Elements must be presorted to deduplicate.");
        // }
        let n : usize = elements.len();
        let (valid , backtrack) = predicate(tmp, start, n);
        if valid {
            result.push(tmp.clone());
        }
        if backtrack {
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

    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::helper(&nums, 0)
    }

}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_78() {
        assert_eq!(Solution::subsets(vec![]), vec![vec![]]);
        assert_eq!(Solution::subsets(vec![1]), vec![vec![], vec![1]]);
        assert_eq!(
            Solution::subsets(vec![1, 2]),
            vec![vec![], vec![2], vec![1], vec![1, 2]]
        );
    }
}
