/**
 * [4] Median of Two Sorted Arrays
 *
 * Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
 *  
 * Example 1:
 * 
 * Input: nums1 = [1,3], nums2 = [2]
 * Output: 2.00000
 * Explanation: merged array = [1,2,3] and median is 2.
 * 
 * Example 2:
 * 
 * Input: nums1 = [1,2], nums2 = [3,4]
 * Output: 2.50000
 * Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 * 
 * Example 3:
 * 
 * Input: nums1 = [0,0], nums2 = [0,0]
 * Output: 0.00000
 * 
 * Example 4:
 * 
 * Input: nums1 = [], nums2 = [1]
 * Output: 1.00000
 * 
 * Example 5:
 * 
 * Input: nums1 = [2], nums2 = []
 * Output: 2.00000
 * 
 *  
 * Constraints:
 * 
 * 	nums1.length == m
 * 	nums2.length == n
 * 	0 <= m <= 1000
 * 	0 <= n <= 1000
 * 	1 <= m + n <= 2000
 * 	-10^6 <= nums1[i], nums2[i] <= 10^6
 * 
 *  
 * Follow up: The overall run time complexity should be O(log (m+n)).
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/median-of-two-sorted-arrays/
// discuss: https://leetcode.com/problems/median-of-two-sorted-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_nth(nums1 : &Vec<i32>, start1 : usize, end1 : usize, 
        nums2: &Vec<i32>, start2 : usize, end2 : usize, n : usize, level : usize) -> i32 {
        // let level_padding : String = (0..level).map(|_|{"  "}).collect();
        let level_padding = "";
        // println!("{}start1={}, end1={}, start2={}, end2={}, n={}", level_padding, start1, end1, start2, end2, n);

        if start1 == end1 {
            return nums2[start2 + n];
        } else if start2 == end2 {
            return nums1[start1 + n];
        } else if n == 0 {
            return std::cmp::min(nums1[start1], nums2[start2]);
        }

        let mid1 : usize = std::cmp::min((n+1)/2, end1-start1);
        let mid2 : usize = std::cmp::min((n+1)/2, end2-start2);
        // println!("nums1[mid={}]={}, nums2[mid={}]={}", start1 + mid1-1,nums1[start1 + mid1-1], start2 +mid2-1, nums2[start2 + mid2-1]);
        // note that mid1 + mid2 <= n. 
        if nums1[start1 + mid1-1] < nums2[start2 + mid2-1] {
            // For any ni = nums1[i] such that 0<=i<mid1, there are i nums in nums1 <= ni, and AT MOST mid2 elements <= ni in nums. 
            // Hence any ni can never be the n-th element. 
            return Self::find_nth(nums1, start1 + mid1, end1, nums2, start2, end2, n - mid1, level + 1);
        } else {
            return Self::find_nth(nums1, start1, end1, nums2, start2 + mid2, end2, n - mid2, level + 1);
        }
    }
    // start inclusive
    // end exclusive
    pub fn my_find_nth(mut nums1 : &Vec<i32>, start1 : usize, end1 : usize, 
        mut nums2: &Vec<i32>, start2 : usize, end2 : usize, n : usize, level : usize) -> i32 {

        let level_padding : String = (0..level).map(|_|{"  "}).collect();
        let mut l1 : usize = end1 - start1;
        let mut l2 : usize = end2 - start2;
        if l1 < l2 {
            return Self::find_nth(nums2, start2, end2, nums1, start1, end1, n,level);
        }
        
        let mid1_num : i32 = nums1[start1 + n / 2];
        // println!("{}nums1={:?}, nums2={:?}", level_padding, nums1, nums2);
        // println!("{}start1={}, end1={}, mid1_pos={}, mid1_num={}, start2={}, end2={},n={}", level_padding, start1, end1, start1 + n / 2, mid1_num, start2, end2, n);
        // find the index of last smaller element in num2, -1 implies not found. 
        let mut mid2_pos : i32 = -1;
        let mut low : i32 = 0;
        let mut high : i32 = l2 as i32 - 1;
        while low <= high {
            let cur_mid : i32 = (high + low) / 2;
            let cur_mid2_num : i32 = nums2[start2 + cur_mid as usize];
            // println!("{}low={}, high={}, cur_mid={}, cur_mid2_num={}", level_padding, low, high, cur_mid, cur_mid2_num);
            if cur_mid2_num <= mid1_num {
                if cur_mid as usize == l2  - 1 {
                    // println!("cur_mid={},l2={}",cur_mid,l2);
                    mid2_pos = cur_mid;
                    break;
                } else if !(nums2[start2 + cur_mid as usize + 1] <= mid1_num) {
                    // find the target
                    mid2_pos = cur_mid;
                    break;
                } else {
                    low = cur_mid + 1;
                }
            } else {
                high = cur_mid - 1;
            }
        }
        let mid1_num_le_count : usize = (mid2_pos + 1) as usize + n /2;
        // println!("{}mid2_pos={}, mid1_num_le_count={}", level_padding,mid2_pos, mid1_num_le_count);
        if mid1_num_le_count == n {
            return mid1_num;
        } else if mid1_num_le_count < n {
            return Self::find_nth(nums1, start1 + n / 2 + 1, end1, nums2, start2 + (mid2_pos + 1) as usize, end2, n - 1 - mid1_num_le_count, level+1);
        } else {
            return Self::find_nth(nums1, start1, start1 + n / 2, nums2, start2, start2 + (mid2_pos + 1) as usize, n,level + 1);
        }
    }
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let l1 : usize = nums1.len();
        let l2 : usize = nums2.len();
        if (l1 + l2) % 2 == 1 {
            let idx : usize = (l1 + l2 - 1) / 2;
            let mid : i32 = Self::find_nth(&nums1, 0, l1, &nums2, 0, l2, idx,0); 
            println!("Odd {}-th = {}", idx, mid);
            return mid as f64;
        } else {
            let idx : usize = (l1 + l2) / 2 - 1;
            let smaller_mid : i32 = Self::find_nth(&nums1, 0, l1, &nums2, 0, l2, idx,0); 
            println!("Even {}-th = {}", idx, smaller_mid);
            println!("=======================================");

            let idx : usize = (l1 + l2) / 2;
            let larger_mid : i32 = Self::find_nth(&nums1, 0, l1, &nums2, 0, l2, idx,0); 
            println!("Even {}-th = {}", idx, larger_mid);
            return (smaller_mid + larger_mid) as f64 / 2.0;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );

        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![1, 1]),
            1.0
        );

        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 7]),
            2.5
        );

        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1], vec![2, 3, 4]),
            2.5
        );
    }
}
