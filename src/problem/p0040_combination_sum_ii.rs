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
    pub fn helper(result: &mut Vec<Vec<i32>>, tmp: &mut Vec<i32>, nums: &Vec<i32>, start:usize, remaining_target: i32) {
        if remaining_target == 0 {
            result.push(tmp.clone());
            return;
        } else if remaining_target < 0 {
            return;
        }
        for i in start..nums.len() {
            if start < i && nums[i] == nums[i-1] {
                continue;
            }
            tmp.push(nums[i]);
            Self::helper(result, tmp, nums, i + 1, remaining_target - nums[i]);
            tmp.pop();
        }
    }

    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result =  vec![];
        let mut tmp : Vec<i32> =  vec![];
        candidates.sort();
        Self::helper(&mut result, &mut tmp, &candidates, 0, target);
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
            vec![vec![7, 1], vec![6, 2], vec![6, 1, 1], vec![5, 2, 1],]
        );
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![5], vec![2, 2, 1],]
        );
    }
}
