/**
 * [154] Find Minimum in Rotated Sorted Array II
 *
 * Suppose an array of length n sorted in ascending order is rotated between 1 and n times. For example, the array nums = [0,1,4,4,5,6,7] might become:
 * 
 * 	[4,5,6,7,0,1,4] if it was rotated 4 times.
 * 	[0,1,4,4,5,6,7] if it was rotated 7 times.
 * 
 * Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].
 * Given the sorted rotated array nums that may contain duplicates, return the minimum element of this array.
 *  
 * Example 1:
 * Input: nums = [1,3,5]
 * Output: 1
 * Example 2:
 * Input: nums = [2,2,2,0,1]
 * Output: 0
 *  
 * Constraints:
 * 
 * 	n == nums.length
 * 	1 <= n <= 5000
 * 	-5000 <= nums[i] <= 5000
 * 	nums is sorted and rotated between 1 and n times.
 * 
 *  
 * Follow up: This is the same as <a href="https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/description/" target="_blank">Find Minimum in Rotated Sorted Array</a> but with duplicates. Would allow duplicates affect the run-time complexity? How and why?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/
// discuss: https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {

    pub fn find_min(mut nums: Vec<i32>) -> i32 {
        let mut min : i32 = 5000;
        let mut left : i32 = 0i32;
        let mut right : i32 = nums.len() as i32 - 1;
        while left <= right {
            let mid : i32 = (left + right) / 2;
            let left_num = nums[left as usize];
            let mid_num = nums[mid as usize];
            let right_num = nums[right as usize];

            if left_num < mid_num {
                //[left, mid] is non-decreasing
                min = std::cmp::min(min, left_num);
                left = mid + 1;
            } else if left_num > mid_num {
                //[mid, right] is non-decreasing
                min = std::cmp::min(min, mid_num);
                right = mid - 1;
            } else {
                min = std::cmp::min(min, left_num);
                left += 1;
            }
        }
        min
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_154() {
        assert_eq!(Solution::find_min(vec![1, 2, 2, 2, 2, 2]), 1);
        assert_eq!(Solution::find_min(vec![1, 3, 3]), 1);
        assert_eq!(Solution::find_min(vec![3, 1, 3, 3]), 1);
        assert_eq!(Solution::find_min(vec![1, 1]), 1);
        assert_eq!(Solution::find_min(vec![3, 3, 3, 1]), 1);
        assert_eq!(Solution::find_min(vec![3, 1, 1]), 1);
    }
}
