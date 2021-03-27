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
    pub fn helper(nums: &Vec<i32>, pos: usize) -> Vec<Vec<i32>> {
        // println!("=========pos: {}================", pos);
        let mut results = vec![vec![]];
        for i in pos..nums.len() {
            // ignore dup
            if pos < i && nums[i] == nums[i-1] {
                continue;
            }
            let mut prev_subsets = Self::helper(nums, i+1);
            // println!("   pos: {}, i: {}, prev_subsets: {:?}", pos, i, prev_subsets);
            for mut prev_subset in prev_subsets {
                let mut my_subset = vec![nums[i]];
                my_subset.append(&mut prev_subset);
                results.push(my_subset);
            }
        }
        // println!("pos: {}, subsets: {:?}", pos, results);
        // println!("=========END pos: {}================", pos);
        results
    }

    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        Self::helper(&nums, 0)
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
                vec![2],
                vec![2, 2],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
            ]
        );
        assert_eq!(Solution::subsets_with_dup(vec![1]), vec![vec![], vec![1],]);
        assert_eq!(Solution::subsets_with_dup(vec![]), vec![vec![],]);
    }
}
