/**
 * [189] Rotate Array
 *
 * Given an array, rotate the array to the right by k steps, where k is non-negative.
 *  
 * Example 1:
 * 
 * Input: nums = [1,2,3,4,5,6,7], k = 3
 * Output: [5,6,7,1,2,3,4]
 * Explanation:
 * rotate 1 steps to the right: [7,1,2,3,4,5,6]
 * rotate 2 steps to the right: [6,7,1,2,3,4,5]
 * rotate 3 steps to the right: [5,6,7,1,2,3,4]
 * 
 * Example 2:
 * 
 * Input: nums = [-1,-100,3,99], k = 2
 * Output: [3,99,-1,-100]
 * Explanation: 
 * rotate 1 steps to the right: [99,-1,-100,3]
 * rotate 2 steps to the right: [3,99,-1,-100]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	0 <= k <= 10^5
 * 
 *  
 * Follow up:
 * 
 * 	Try to come up with as many solutions as you can. There are at least three different ways to solve this problem.
 * 	Could you do it in-place with O(1) extra space?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rotate-array/
// discuss: https://leetcode.com/problems/rotate-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // start inclusive, end exclusive
    pub fn reverse(nums: &mut Vec<i32>, start : usize, end : usize) {
        let mid = (end - start) / 2;
        for i in 0..mid {
            nums.swap(start + i, end - 1 - i);
        }
    }

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k : usize = k as usize % nums.len();
        Self::reverse(nums, 0, nums.len()); 
        Self::reverse(nums, 0, k); 
        Self::reverse(nums, k, nums.len()); 
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_189() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
        let mut nums = vec![1, 2, 3, 4, 5, 6];
        Solution::rotate(&mut nums, 2);
        assert_eq!(nums, vec![5, 6, 1, 2, 3, 4]);
    }
}
