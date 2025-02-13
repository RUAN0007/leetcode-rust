/**
 * [162] Find Peak Element
 *
 * A peak element is an element that is strictly greater than its neighbors.
 * Given an integer array nums, find a peak element, and return its index. If the array contains multiple peaks, return the index to any of the peaks.
 * You may imagine that nums[-1] = nums[n] = -&infin;.
 *  
 * Example 1:
 * 
 * Input: nums = [1,2,3,1]
 * Output: 2
 * Explanation: 3 is a peak element and your function should return the index number 2.
 * Example 2:
 * 
 * Input: nums = [1,2,1,3,5,6,4]
 * Output: 5
 * Explanation: Your function can return either index number 1 where the peak element is 2, or index number 5 where the peak element is 6.
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 1000
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	nums[i] != nums[i + 1] for all valid i.
 * 
 *  
 * Follow up: Could you implement a solution with logarithmic complexity?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-peak-element/
// discuss: https://leetcode.com/problems/find-peak-element/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn helper(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        // nums has at least 2 elements. 
        println!("start={},  end={}", start, end);
        if start >= end {
            return -1; // not found. 
        }

        let mid = (start + end) / 2;

        if mid == 0 {
            if nums[mid] > nums[mid+1] {
                return mid as i32;
            } else {
                return Self::helper(nums, mid+1, end);
            }
        } else if mid == nums.len() - 1 {
            if nums[mid-1] < nums[mid] {
                return mid as i32;
            } else {
                return Self::helper(nums, start, mid);
            }
        } else {
            if nums[mid-1] < nums[mid] && nums[mid] > nums[mid+1] {
                return mid as i32;
            } else if nums[mid-1] > nums[mid] {
                return Self::helper(nums, start, mid);
            } else if nums[mid] < nums[mid+1] && mid + 1 < end {
                return Self::helper(nums, mid+1, end);
            } else if nums[mid-1] == nums[mid] {
                let left = Self::helper(nums, start, mid); 
                if left != -1 {
                    return left;
                } else {
                    return Self::helper(nums, mid+1, end);
                }
            }
            panic!("Shall not reach here");
        }
    }

    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0
        } else {
            return Self::helper(&nums, 0, nums.len());
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_162() {
        // assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
        assert_eq!(Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 5);
    }
}
