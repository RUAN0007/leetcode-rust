/**
 * [321] Create Maximum Number
 *
 * You are given two integer arrays nums1 and nums2 of lengths m and n respectively. nums1 and nums2 represent the digits of two numbers. You are also given an integer k.
 * Create the maximum number of length k <= m + n from digits of the two numbers. The relative order of the digits from the same array must be preserved.
 * Return an array of the k digits representing the answer.
 *  
 * Example 1:
 * 
 * Input: nums1 = [3,4,6,5], nums2 = [9,1,2,5,8,3], k = 5
 * Output: [9,8,6,5,3]
 * 
 * Example 2:
 * 
 * Input: nums1 = [6,7], nums2 = [6,0,4], k = 5
 * Output: [6,7,6,0,4]
 * 
 * Example 3:
 * 
 * Input: nums1 = [3,9], nums2 = [8,9], k = 3
 * Output: [9,8,9]
 * 
 *  
 * Constraints:
 * 
 * 	m == nums1.length
 * 	n == nums2.length
 * 	1 <= m, n <= 500
 * 	0 <= nums1[i], nums2[i] <= 9
 * 	1 <= k <= m + n
 * 
 *  
 * Follow up: Try to optimize your time and space complexity.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/create-maximum-number/
// discuss: https://leetcode.com/problems/create-maximum-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn max_num(nums : &Vec<i32>, k : usize) -> Vec<i32> {
        let mut result : Vec<i32> = vec![];
        let n : usize = nums.len();
        for (i, &num) in nums.iter().enumerate() {
            while result.len() != 0 && result.len() + (n-i) > k {
                if *result.last().unwrap() < num {
                    result.pop();
                } else {
                    break;
                }
            }

            if result.len() < k {
                result.push(num);
            }
        }
        // println!("nums={:?}, k={}, result={:?}", nums, k, result);
        result
    }

    pub fn is_nums1_greater(nums1 : &Vec<i32>, nums1_start : usize, nums2 : &Vec<i32>, nums2_start : usize) -> bool {
        let mut i = nums1_start;
        let mut j = nums2_start;
        while i < nums1.len() && j < nums2.len() && nums1[i] == nums2[j] {
            i += 1;
            j += 1;
        }
        j == nums2.len() || i < nums1.len() && nums1[i] >  nums2[j]
    }

    pub fn merge(nums1 : &Vec<i32>, nums2 : &Vec<i32>) -> Vec<i32> {
        let mut result : Vec<i32> = vec![];
        let mut i : usize = 0;
        let mut j : usize = 0;
        while result.len() != nums1.len() + nums2.len() {
            if Self::is_nums1_greater(nums1, i, nums2, j) {
                result.push(nums1[i]);
                i+=1;
            } else {
                result.push(nums2[j]);
                j+=1;
            }
        }
        result
    }

    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let k : usize = k as usize;
        let mut ans = vec![];
        let min_from_num1 : usize = if k > nums2.len() {k - nums2.len()} else {0};
        let max_from_num1 : usize = if k > nums1.len() {nums1.len()} else {k};

        for i in (min_from_num1..=max_from_num1) {
            let candidate = Self::merge(&Self::max_num(&nums1, i), &Self::max_num(&nums2, k-i));
            if Self::is_nums1_greater(&candidate, 0, &ans, 0) {
                ans = candidate;
            }
        }
        ans
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_321() {
        assert_eq!(Solution::max_number(vec![3,4,6,5], vec![9,1,2,5,8,3], 5), vec![9,8,6,5,3]);
        assert_eq!(Solution::max_number(vec![6,7], vec![6,0,4], 5), vec![6,7,6,0,4]);
        assert_eq!(Solution::max_number(vec![3,9], vec![8,9], 3), vec![9,8,9]);
    }
}
