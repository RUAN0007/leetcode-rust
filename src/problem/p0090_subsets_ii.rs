/**
 * [90] Subsets II
 *
 * Given an integer array nums that may contain duplicates, return all possible subsets (the power set).
 * The solution set must not contain duplicate subsets. Return the solution in any order.
 *  
 * Example 1:
 * Input: nums = [1,2,2]
 * Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]
 * Example 2:
 * Input: nums = [0]
 * Output: [[],[0]]
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10
 * 	-10 <= nums[i] <= 10
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/subsets-ii/
// discuss: https://leetcode.com/problems/subsets-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn backtrack_helper<P>(result : &mut Vec<Vec<i32>>, tmp : &mut Vec<i32>, elements : &Vec<i32>, predicate: P, start : usize, no_dup : bool, element_reusable : bool) where P:Fn(&Vec<i32>)->(bool, bool) + Copy {
        // is_sorted() is only supported in nightly-built rust
        // if no_dup && !elements.is_sorted() {
        //     panic!("Elements must be presorted to deduplicate.");
        // }
        let n : usize = elements.len();
        let (valid , backtrack) = predicate(tmp);
        if valid {
            result.push(tmp.clone());
        }
        if backtrack {
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

    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result : Vec<Vec<i32>> = vec![];
        let mut tmp : Vec<i32> = vec![];
        let element_reusable = false;
        let no_dup = true;
        nums.sort();

        let predicate = |tmp : &Vec<i32>|{ (true, true) };
        Self::backtrack_helper(&mut result, &mut tmp, &nums, predicate, 0, no_dup, element_reusable);
        result
    }

}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_90() {
        assert_eq!(
            Solution::subsets_with_dup(vec![1, 2, 2]),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2],
            ]
        );
        assert_eq!(Solution::subsets_with_dup(vec![1]), vec![vec![], vec![1],]);
        assert_eq!(Solution::subsets_with_dup(vec![]), vec![vec![],]);
    }
}
