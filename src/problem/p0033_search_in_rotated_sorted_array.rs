/**
 * [33] Search in Rotated Sorted Array
 *
 * There is an integer array nums sorted in ascending order (with distinct values).
 * Prior to being passed to your function, nums is rotated at an unknown pivot index k (0 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].
 * Given the array nums after the rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.
 *  
 * Example 1:
 * Input: nums = [4,5,6,7,0,1,2], target = 0
 * Output: 4
 * Example 2:
 * Input: nums = [4,5,6,7,0,1,2], target = 3
 * Output: -1
 * Example 3:
 * Input: nums = [1], target = 0
 * Output: -1
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 5000
 * 	-10^4 <= nums[i] <= 10^4
 * 	All values of nums are unique.
 * 	nums is guaranteed to be rotated at some pivot.
 * 	-10^4 <= target <= 10^4
 * 
 *  
 * Follow up: Can you achieve this in O(log n) time complexity?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/search-in-rotated-sorted-array/
// discuss: https://leetcode.com/problems/search-in-rotated-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low : i32 = 0;
        let mut high : i32 = nums.len() as i32 - 1;
        while low <= high {
            let mid : i32 = (low + high) / 2;
            let mid_num : i32 = nums[mid as usize];
            let right_num : i32 = nums[high as usize];
            let left_num : i32 = nums[low as usize];
            // println!("low[{}]={}, mid[{}]={}, right[{}]={}", low, left_num, mid, mid_num, high, right_num);
            if mid_num == target {
                return mid;
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
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_33() {
        // assert_eq!(Solution::search(vec![7, 8, 1, 2, 3, 4, 5, 6], 2), 3);
        // assert_eq!(
        //     Solution::search(
        //         vec![
        //             1004, 1005, 1006, 1007, 1008, 1009, 1010, 1011, 1012, 0, 1, 2, 3, 4, 5, 6, 7, 8
        //         ],
        //         0
        //     ),
        //     9
        // );
        // assert_eq!(
        //     Solution::search(
        //         vec![
        //             1004, 1005, 1006, 1007, 1008, 1009, 1010, 1011, 1012, 0, 1, 2, 3, 4, 5, 6, 7, 8
        //         ],
        //         1006
        //     ),
        //     2
        // );
        // assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        // assert_eq!(Solution::search(vec![], 3), -1);
        assert_eq!(Solution::search(vec![5,1,3], 5), 0);
    }
}
