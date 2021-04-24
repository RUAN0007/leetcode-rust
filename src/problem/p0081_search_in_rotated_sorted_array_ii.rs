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

    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut low : i32 = 0;
        let mut high : i32 = nums.len() as i32 - 1;
        while low <= high {
            let mid : i32 = (low + high) / 2;
            let mid_num : i32 = nums[mid as usize];
            let right_num : i32 = nums[high as usize];
            let left_num : i32 = nums[low as usize];
            // println!("low[{}]={}, mid[{}]={}, right[{}]={}", low, left_num, mid, mid_num, high, right_num);
            if mid_num == target {
                return true;
            } else if left_num < mid_num {
                //[low, mid] must be sorted. 
                if left_num <= target && target < mid_num {
                    // in the ordered left half
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            } else if left_num > mid_num {
                // [mid, high] must be sorted.
                if mid_num < target && target <= right_num {
                    // in the ordered right half
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            } else {
                low += 1;
            }
        }
        false
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

        assert_eq!(
            Solution::search(vec![1,0,1,1,1], 0),
           true 
        );

    }
}
