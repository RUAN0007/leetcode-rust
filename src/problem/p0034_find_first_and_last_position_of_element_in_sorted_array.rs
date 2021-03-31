/**
 * [34] Find First and Last Position of Element in Sorted Array
 *
 * Given an array of integers nums sorted in ascending order, find the starting and ending position of a given target value.
 * If target is not found in the array, return [-1, -1].
 * Follow up: Could you write an algorithm with O(log n) runtime complexity?
 *  
 * Example 1:
 * Input: nums = [5,7,7,8,8,10], target = 8
 * Output: [3,4]
 * Example 2:
 * Input: nums = [5,7,7,8,8,10], target = 6
 * Output: [-1,-1]
 * Example 3:
 * Input: nums = [], target = 0
 * Output: [-1,-1]
 *  
 * Constraints:
 * 
 * 	0 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 * 	nums is a non-decreasing array.
 * 	-10^9 <= target <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
// discuss: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Return -1 if no such element found. 
    // end_pos exclusive
    pub fn last_smaller(nums: &Vec<i32>, start_pos: usize, end_pos: usize, target: i32) -> i32 {
        if target <= nums[start_pos] {
            -1
        } else if start_pos == end_pos - 1 {
            start_pos as i32
        } else {
            let mid = (start_pos + end_pos) / 2;
            let mid_num = nums[mid as usize];
            if target <= mid_num {
                Self::last_smaller(nums, start_pos, mid, target)
            } else {
                Self::last_smaller(nums, mid, end_pos, target)
            }
        }
    }

    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }

        let high_idx = Self::last_smaller(&nums, 0, nums.len(), target+1);

        if high_idx != -1 && nums[high_idx as usize] == target {
            // low_idx can be -1
            let low_idx = Self::last_smaller(&nums, 0, nums.len(), target);
            vec![low_idx+1, high_idx]
        } else {
            vec![-1, -1]
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_34() {
        assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 8), vec![3,4]);
        assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 6), vec![-1,-1]);
        assert_eq!(Solution::search_range(vec![], 0), vec![-1,-1]);

    }
}
