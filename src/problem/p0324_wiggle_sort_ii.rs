/**
 * [324] Wiggle Sort II
 *
 * Given an integer array nums, reorder it such that nums[0] < nums[1] > nums[2] < nums[3]....
 * You may assume the input array always has a valid answer.
 *  
 * Example 1:
 * 
 * Input: nums = [1,5,1,1,6,4]
 * Output: [1,6,1,5,1,4]
 * Explanation: [1,4,1,5,1,6] is also accepted.
 * 
 * Example 2:
 * 
 * Input: nums = [1,3,2,2,3,1]
 * Output: [2,3,1,3,1,2]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 5 * 10^4
 * 	0 <= nums[i] <= 5000
 * 	It is guaranteed that there will be an answer for the given input nums.
 * 
 *  
 * Follow Up: Can you do it in O(n) time and/or in-place with O(1) extra space?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/wiggle-sort-ii/
// discuss: https://leetcode.com/problems/wiggle-sort-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {

    fn find_kth(nums : &Vec<i32>, k: usize) -> i32 {
        let num1 = nums[0];
        let mut lt_nums = vec![];
        let mut ge_nums = vec![];
        for i in 1..nums.len() {
            if nums[i] < num1 {
                lt_nums.push(nums[i]);
            } else {
                ge_nums.push(nums[i]);
            }
        }

        if lt_nums.len() == k {
            num1
        }  else if lt_nums.len() < k {
            Self::find_kth(&ge_nums, k - lt_nums.len() - 1) 
        } else {
            Self::find_kth(&lt_nums, k)
        }
    }
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        if nums.len() < 2 {return}
        let n = nums.len();
        let median = Self::find_kth(nums, (nums.len() - 1)/2);
        // println!("median = {}", median);
        let idx_map : Vec<usize> = (0..nums.len()).map(|x| {(1+2*x) % (n | 1)}).collect();
        // println!("idx_map={:?}", idx_map);
        let mut left = 0;
        let mut i = 0;
        let mut right = n - 1;
        while i <= right {
            if nums[idx_map[i]] > median {
                nums.swap(idx_map[i], idx_map[left]);
                i+=1;
                left+=1;
            } else if nums[idx_map[i]] < median {
                nums.swap(idx_map[i], idx_map[right]);
                right-=1;
            } else {
                i+=1;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_324() {
        let mut r = vec![1,5,1,1,6,4];
        Solution::wiggle_sort(&mut r);
        assert_eq!(r, vec![1,6,1,5,1,4]);
    }
}
