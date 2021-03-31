/**
 * [81] Search in Rotated Sorted Array II
 *
 * There is an integer array nums sorted in non-decreasing order (not necessarily with distinct values).
 * Before being passed to your function, nums is rotated at an unknown pivot index k (0 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,4,4,5,6,6,7] might be rotated at pivot index 5 and become [4,5,6,6,7,0,1,2,4,4].
 * Given the array nums after the rotation and an integer target, return true if target is in nums, or false if it is not in nums.
 *  
 * Example 1:
 * Input: nums = [2,5,6,0,0,1,2], target = 0
 * Output: true
 * Example 2:
 * Input: nums = [2,5,6,0,0,1,2], target = 3
 * Output: false
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 5000
 * 	-10^4 <= nums[i] <= 10^4
 * 	nums is guaranteed to be rotated at some pivot.
 * 	-10^4 <= target <= 10^4
 * 
 *  
 * Follow up: This problem is the same as <a href="/problems/search-in-rotated-sorted-array/description/" target="_blank">Search in Rotated Sorted Array</a>, where nums may contain duplicates. Would this affect the runtime complexity? How and why?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/search-in-rotated-sorted-array-ii/
// discuss: https://leetcode.com/problems/search-in-rotated-sorted-array-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {

    pub fn helper(nums: &Vec<i32>, start: usize, end: usize, target: i32) -> bool {

        if start >= end {
            return false;
        }

        let mid = (start + end) / 2;
        let mid_num = nums[mid];
        // println!("start: nums[{}]={}, end: nums[{}]={}, mid: nums[{}]={}", start, nums[start], end-1, nums[end-1], mid, mid_num);
        if mid_num == target {
            return true;
        } else if nums[start] <= target && target < nums[mid] {
            // println!(" In First Increasing Interval");
            return Self::helper(nums, start, mid, target);
        } else if nums[mid] < target && target <= nums[end-1] {
            // println!(" In Second Increasing Interval");
            return Self::helper(nums, mid+1, end, target);
        } 
        
        let mut result = false; 
        if nums[start] >= nums[mid] && (target < nums[mid] || nums[start] <= target) {
            // println!(" In First Non-increasing Interval");
            result = Self::helper(nums, start, mid, target)
        } 
        
        if result {return true; }
        
        if nums[mid] >= nums[end-1] && (target <= nums[end-1] || nums[mid]< target) {
            // println!(" In Second Non-increasing Interval");
            result = Self::helper(nums, mid+1, end, target)
        }
        result
    }

    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut start = 0usize;  // inclusive
        let mut end = nums.len(); // exclusive
        Self::helper(&nums, start, end, target)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_81() {
        assert_eq!(
            Solution::search(vec![1,1,1,1,1,1,1,1,1,1,1,1,1,2,1,1,1,1,1], 2),
            true
        );

        assert_eq!(
            Solution::search(vec![1,1], 0),
           false 
        );

    }
}
