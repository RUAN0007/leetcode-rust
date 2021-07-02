/**
 * [377] Combination Sum IV
 *
 * Given an array of distinct integers nums and a target integer target, return the number of possible combinations that add up to target.
 * The answer is guaranteed to fit in a 32-bit integer.
 *  
 * Example 1:
 * 
 * Input: nums = [1,2,3], target = 4
 * Output: 7
 * Explanation:
 * The possible combination ways are:
 * (1, 1, 1, 1)
 * (1, 1, 2)
 * (1, 2, 1)
 * (1, 3)
 * (2, 1, 1)
 * (2, 2)
 * (3, 1)
 * Note that different sequences are counted as different combinations.
 * 
 * Example 2:
 * 
 * Input: nums = [9], target = 3
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 200
 * 	1 <= nums[i] <= 1000
 * 	All the elements of nums are unique.
 * 	1 <= target <= 1000
 * 
 *  
 * Follow up: What if negative numbers are allowed in the given array? How does it change the problem? What limitation we need to add to the question to allow negative numbers?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/combination-sum-iv/
// discuss: https://leetcode.com/problems/combination-sum-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn helper(nums : &Vec<i32>, target : i32, cache : &mut HashMap<i32, i32>) -> i32 {
        if target < 0 {
            0
        } else if target == 0 {
            1
        } else if let Some(&cached) = cache.get(&target) {
            cached
        } else {
            let mut r : i32 = 0;
            for &num in nums.iter() {
                r += Self::helper(nums, target - num, cache);
            }
            cache.insert(target, r);
            r
        }
    }

    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        Self::helper(&nums, target, &mut HashMap::new())
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_377() {
        assert_eq!(Solution::combination_sum4(vec![1,2,3], 4), 7);
        assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
    }
}
