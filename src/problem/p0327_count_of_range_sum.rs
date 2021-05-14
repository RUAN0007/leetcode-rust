/**
 * [327] Count of Range Sum
 *
 * Given an integer array nums and two integers lower and upper, return the number of range sums that lie in [lower, upper] inclusive.
 * Range sum S(i, j) is defined as the sum of the elements in nums between indices i and j inclusive, where i <= j.
 *  
 * Example 1:
 * 
 * Input: nums = [-2,5,-1], lower = -2, upper = 2
 * Output: 3
 * Explanation: The three ranges are: [0,0], [2,2], and [0,2] and their respective sums are: -2, -1, 2.
 * 
 * Example 2:
 * 
 * Input: nums = [0], lower = 0, upper = 0
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	-10^5 <= lower <= upper <= 10^5
 * 	The answer is guaranteed to fit in a 32-bit integer.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-of-range-sum/
// discuss: https://leetcode.com/problems/count-of-range-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn last_lt_idx(nums : &Vec<i64>, base : i64, sorted_idx : &Vec<usize>, target : i64) -> i32 {
        let mut lower : i32 = 0;
        let mut higher : i32 = sorted_idx.len() as i32 - 1;
        while lower <= higher {
            let valid = |i : usize|{nums[sorted_idx[i]] - base < target};
            let mid = (lower + higher) / 2;
            if valid(mid as usize) {
                if mid as usize == sorted_idx.len() - 1 || !valid(mid as usize + 1) {
                    return mid;
                } else {
                    lower = mid + 1;
                }
            } else {
                higher = mid - 1;
            }
        }
        -1 as i32
    }

    pub fn last_le_idx(nums : &Vec<i64>, base : i64, sorted_idx : &Vec<usize>, target : i64) -> i32 {
        let mut lower : i32 = 0;
        let mut higher : i32 = sorted_idx.len() as i32 - 1;
        let valid = |i : usize|{nums[sorted_idx[i]] - base <= target};
        while lower <= higher {
            // println!("lower={}, higher={}, sorted_idx={:?}", lower, higher, sorted_idx);
            let mid = (lower + higher) / 2;
            if valid(mid as usize) {
                if mid as usize == sorted_idx.len() - 1 || !valid(mid as usize + 1) {
                    return mid;
                } else {
                    lower = mid + 1;
                }
            } else {
                higher = mid - 1;
            }
        }
        -1 as i32
    }

    
    pub fn count_in_mergesort_index(counts : &mut Vec<i32>, nums : &Vec<i64>, idx_to_sort : &Vec<usize>, lower : i64, higher : i64) -> Vec<usize> {
        let n : usize = idx_to_sort.len();
        if n > 1 {
            let mut idx_to_sort1 : Vec<usize> = idx_to_sort.clone();
            let idx_to_sort2 : Vec<usize> = idx_to_sort1.split_off(n/2);
            let left_idx_sorted : Vec<usize> = Self::count_in_mergesort_index(counts, nums, &idx_to_sort1, lower, higher);
            let right_idx_sorted : Vec<usize> = Self::count_in_mergesort_index(counts, nums, &idx_to_sort2, lower, higher);

            let mut left_i : usize = 0;
            let left_count : usize = left_idx_sorted.len();

            let mut right_i : usize = 0;
            let right_count : usize = right_idx_sorted.len();
            let mut merged_idx : Vec<usize> = vec![];
            while left_i != left_count || right_i != right_count {
                if right_i == right_count || (left_i != left_count && nums[left_idx_sorted[left_i]] < nums[right_idx_sorted[right_i]]) {

                    let mut right_lt_lower_count : i32 = Self::last_lt_idx(&nums, nums[left_idx_sorted[left_i]], &right_idx_sorted, lower) + 1;
                    // let mut right_lt_lower_count : i32 = 0;
                    // for &right_idx in right_idx_sorted.iter() {
                    //     if nums[right_idx] - nums[left_idx_sorted[left_i]] < lower {
                    //         right_lt_lower_count+=1;
                    //     } else {
                    //         break;
                    //     }
                    // }

                    let mut right_le_higher_count : i32 = Self::last_le_idx(&nums, nums[left_idx_sorted[left_i]], &right_idx_sorted, higher) + 1;
                    // let mut right_le_higher_count : i32 = 0;
                    // for &right_idx in right_idx_sorted.iter() {
                    //     if nums[right_idx] - nums[left_idx_sorted[left_i]] <= higher  {
                    //         right_le_higher_count+=1;
                    //     } else {
                    //         break;
                    //     }
                    // }
                    counts[left_idx_sorted[left_i]] += right_le_higher_count - right_lt_lower_count;

                    merged_idx.push(left_idx_sorted[left_i]);
                    left_i+=1;
                } else {
                    merged_idx.push(right_idx_sorted[right_i]);
                    right_i+=1;
                }
            }

            return merged_idx;


        } else {
            return idx_to_sort.clone();
        }
    }

    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut prefix_sum : Vec<i64> = vec![0];
        let mut cur_sum : i64 = 0;
        for &num in nums.iter() {
            cur_sum += num as i64;
            prefix_sum.push(cur_sum);
        }
        let n : usize = nums.len();
        let mut right_inrange_count : Vec<i32> = vec![0; n+1];
        let init_idx : Vec<usize> = (0..=n).collect();
        Self::count_in_mergesort_index(&mut right_inrange_count, &prefix_sum, &init_idx, lower as i64, upper as i64);
        // println!("right_inrange_count={:?}", right_inrange_count);
        right_inrange_count.iter().sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_327() {
        assert_eq!(Solution::count_range_sum(vec![-2,5,-1], -2,2), 3);
        assert_eq!(Solution::count_range_sum(vec![0], 0,0), 1);
    }
}
