/**
 * [496] Next Greater Element I
 *
 * You are given two integer arrays nums1 and nums2 both of unique elements, where nums1 is a subset of nums2.
 * Find all the next greater numbers for nums1's elements in the corresponding places of nums2.
 * The Next Greater Number of a number x in nums1 is the first greater number to its right in nums2. If it does not exist, return -1 for this number.
 *  
 * Example 1:
 * 
 * Input: nums1 = [4,1,2], nums2 = [1,3,4,2]
 * Output: [-1,3,-1]
 * Explanation:
 * For number 4 in the first array, you cannot find the next greater number for it in the second array, so output -1.
 * For number 1 in the first array, the next greater number for it in the second array is 3.
 * For number 2 in the first array, there is no next greater number for it in the second array, so output -1.
 * Example 2:
 * 
 * Input: nums1 = [2,4], nums2 = [1,2,3,4]
 * Output: [3,-1]
 * Explanation:
 * For number 2 in the first array, the next greater number for it in the second array is 3.
 * For number 4 in the first array, there is no next greater number for it in the second array, so output -1.
 *  
 * Constraints:
 * 
 * 	1 <= nums1.length <= nums2.length <= 1000
 * 	0 <= nums1[i], nums2[i] <= 10^4
 * 	All integers in nums1 and nums2 are unique.
 * 	All the integers of nums1 also appear in nums2.
 * 
 *  
 * Follow up: Could you find an O(nums1.length + nums2.length) solution?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/next-greater-element-i/
// discuss: https://leetcode.com/problems/next-greater-element-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let mut all_result = HashMap::new();

        let mut stack = vec![];
        nums2.reverse();
        for num in nums2 {
            while let Some(&last) = stack.last() {
                if last < num {
                    stack.pop();
                } else {
                    break;
                }
            }
            if let Some(&last) = stack.last() {
                all_result.insert(num, last);
            } else {
                all_result.insert(num, -1i32);
            }
            stack.push(num); 
        }

        let mut result = vec![];
        for num in nums1 {
            result.push(all_result[&num]);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_496() {

    }
}
