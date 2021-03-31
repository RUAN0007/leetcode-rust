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
    // low, hign are inclusive index. 
    pub fn helper(nums: &Vec<i32>, low: usize, high: usize, target: i32) -> i32 {
        let mid = (low + high) / 2;
        if high - low <= 1 {
            if nums[high] == target {high as i32} 
            else if nums[low] == target {low as i32} else {-1}
        
        // Two possible 1-2-3 pattern, target within an an increasing interval
        } else if nums[low] <= target && target <= nums[mid] {
            Self::helper(nums, low, mid, target)
        } else if nums[mid] <= target && target <= nums[high] {
            Self::helper(nums, mid, high, target)

        // Two possible 3-1-2 pattern or 2-3-1 pattern, target le to the lower bound of an decreasing interval. 
        } else if nums[low] > nums[mid] && (nums[low] <= target || target <= nums[mid]) {
            Self::helper(nums, low, mid, target)
        } else if nums[mid] > nums[high] && (nums[mid] <= target || target <= nums[high]) {
            Self::helper(nums, mid, high, target)

        } else {
            // println!("Impossible pattern with low = {}, mid = {}, high = {}, nums[low]={}, nums[mid]={}, nums[high]={} target={} ", low, mid, high, nums[low], nums[mid], nums[high], target);
           -1 
        }

    }
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return -1;
        }
        Self::helper(&nums, 0, nums.len() - 1, target)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_33() {
        assert_eq!(Solution::search(vec![7, 8, 1, 2, 3, 4, 5, 6], 2), 3);
        assert_eq!(
            Solution::search(
                vec![
                    1004, 1005, 1006, 1007, 1008, 1009, 1010, 1011, 1012, 0, 1, 2, 3, 4, 5, 6, 7, 8
                ],
                0
            ),
            9
        );
        assert_eq!(
            Solution::search(
                vec![
                    1004, 1005, 1006, 1007, 1008, 1009, 1010, 1011, 1012, 0, 1, 2, 3, 4, 5, 6, 7, 8
                ],
                1006
            ),
            2
        );
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![], 3), -1);
    }
}
