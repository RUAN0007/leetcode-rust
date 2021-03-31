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

    pub fn helper(nums: &Vec<i32>, start: usize,end: usize) -> i32 {
        // start is inclusive, end is exclusive
        // println!("start = {}, end = {}", start, end);
        if start >= end {
            panic!("start {} is ge than end {}", start, end);
        }  
        
        if start == end - 1 {
            return nums[start]; // only one valid point
        } 

        //NOTE down start = end - 1
        let mid = (start + end) / 2;
        if 0 < mid && nums[mid-1] > nums[mid] {
            return nums[mid];
        } else if nums[start-1] > nums[mid] {
            return Self::helper(nums, start, mid);
        } else if  nums[mid] > nums[end-1] && mid+1 < end {
            return Self::helper(nums, mid+1, end);
        } 

        let mut this_min = 2147483647; 
        if  nums[start-1] == nums[mid] {
            let left = Self::helper(nums, start, mid);
            this_min = std::cmp::min(this_min, left);
        }

        if nums[mid] == nums[end-1] && mid+1 < end {
            let right = Self::helper(nums, mid+1, end);
            this_min = std::cmp::min(this_min, right);
        }
        return this_min;
    }

    pub fn find_min(mut nums: Vec<i32>) -> i32 {
        // This is to ensure we must find nums[i-1] > nums[i] in case of a sorted vec. 
        nums.push(nums[0]); 
        Self::helper(&nums, 1, nums.len())
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_154() {
        // assert_eq!(Solution::find_min(vec![1, 2, 2, 2, 2, 2]), 1);
        // assert_eq!(Solution::find_min(vec![1, 3, 3]), 1);
        // assert_eq!(Solution::find_min(vec![3, 1, 3, 3]), 1);
        // assert_eq!(Solution::find_min(vec![1, 1]), 1);
        // assert_eq!(Solution::find_min(vec![3, 3, 3, 1]), 1);
        assert_eq!(Solution::find_min(vec![3, 1, 1]), 1);
    }
}
