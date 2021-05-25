/**
 * [493] Reverse Pairs
 *
 * Given an integer array nums, return the number of reverse pairs in the array.
 * A reverse pair is a pair (i, j) where 0 <= i < j < nums.length and nums[i] > 2 * nums[j].
 *  
 * Example 1:
 * Input: nums = [1,3,2,3,1]
 * Output: 2
 * Example 2:
 * Input: nums = [2,4,3,5,1]
 * Output: 3
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 5 * 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-pairs/
// discuss: https://leetcode.com/problems/reverse-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn first_greater(sorted : &Vec<i64>, target : i64) -> i64 {
        let mut low : i64 = 0;
        let mut high : i64 = sorted.len() as i64 - 1;
        while low <= high {
            let mid : usize = ((low + high) / 2 ) as usize;
            if sorted[mid] > target {
                if mid == 0 || !(sorted[mid-1] > target) {
                    return mid as i64;
                } else {
                    high = mid as i64 - 1;
                }
            } else {
                low = mid as i64 + 1;
            }
        }
        sorted.len() as i64
    }

    pub fn count_while_merge_sort(mut nums : Vec<i64>, count : &mut i64) -> Vec<i64> {
        if nums.len() < 2 { return nums }
        let mid : usize = nums.len() / 2;
        let right_half : Vec<i64> = nums.split_off(mid);
        let left_half : Vec<i64> = nums;

        let mut left_half : Vec<i64> = Self::count_while_merge_sort(left_half, count);
        let mut right_half : Vec<i64> = Self::count_while_merge_sort(right_half, count);
        // println!("left={:?}, right={:?}", left_half, right_half);
        for &right_num in right_half.iter() {
            let first_gt_pos : i64 = Self::first_greater(&left_half, 2*right_num);
            // println!("first_gt_pos={}, right_num={}", first_gt_pos, right_num);
            *count += left_half.len() as i64 - first_gt_pos;
        }


        let mut reverse_sorted : Vec<i64> = vec![];
        while left_half.len() != 0 || right_half.len() != 0 {
            if right_half.len() == 0 || left_half.len() != 0 && *left_half.last().unwrap() > *right_half.last().unwrap() {
                reverse_sorted.push(left_half.pop().unwrap());
            } else {
                reverse_sorted.push(right_half.pop().unwrap());
            }
        }
        reverse_sorted.into_iter().rev().collect()
    }

    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let mut count : i64 = 0;
        let nums : Vec<i64> = nums.iter().map(|&x|{x as i64}).collect();
        Self::count_while_merge_sort(nums, &mut count);
        count as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_493() {
        assert_eq!(Solution::reverse_pairs(vec![1,3,2,3,1]), 2);
        assert_eq!(Solution::reverse_pairs(vec![2,4,3,5,1]), 3);
    }
}
