pub struct Solution {}

// problem: https://leetcode.com/problems/find-right-interval/
// discuss: https://leetcode.com/problems/find-right-interval/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn first_equal(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0i32;
        let mut high = (nums.len() - 1) as i32;
        while low <= high {
            let mid = (low + high) / 2;
            let mid_num = nums[mid as usize];
            if mid_num < target {
                low = mid + 1;
            } else if nums[mid as usize] > target {
                high = mid - 1;
            } else if 0 < mid && nums[(mid-1) as usize] == mid_num {
                high = mid - 1;
            } else {
                return mid;
            }
        }
        -1
    }

    pub fn last_equal(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0i32;
        let mut high = (nums.len() - 1) as i32;
        while low <= high {
            let mid = (low + high) / 2;
            let mid_num = nums[mid as usize];
            if mid_num < target {
                low = mid + 1;
            } else if nums[mid as usize] > target {
                high = mid - 1; // if mid is usize and mid=0, this wil panic. 
            } else if (mid as usize) < nums.len() - 1 && nums[(mid+1) as usize] == mid_num {
                low = mid + 1;
            } else {
                return mid;
            }
        }
        -1
    }

    pub fn first_gt(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0i32;
        let mut high = (nums.len() - 1) as i32;
        while low <= high {
            let mid = (low + high) / 2;
            let mid_num = nums[mid as usize];
            // println!("low={}, mid={}, high={}, mid_num={}, target={}", low, mid, high, mid_num, target);
            if target < mid_num {
                if mid == 0 || nums[(mid-1) as usize] <= target {
                    return mid;
                }
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        -1
    }

    pub fn first_ge(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0i32;
        let mut high = (nums.len() - 1) as i32;

        while low <= high {
            let mid = (low + high) / 2;
            let mid_num = nums[mid as usize];
            // println!("low={}, mid={}, high={}, mid_num={}, target={}", low, mid, high, mid_num, target);
            if target <= mid_num {
                if mid == 0 || nums[(mid-1) as usize] < target {
                    return mid;
                }
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        -1
    }

    // assert_eq!(Solution::last_lt(vec![1,1,3,3,5,5],2), 2);
    pub fn last_lt(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0i32;
        let mut high = (nums.len() - 1) as i32;
        let l = nums.len() as i32;
        while low <= high {
            let mid = (low + high) / 2;
            let mid_num = nums[mid as usize];
            // println!("low={}, mid={}, high={}, mid_num={}, target={}", low, mid, high, mid_num, target);
            if mid_num < target {
                if mid == l - 1 || target <= nums[(mid+1) as usize] {
                    return mid;
                }
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        -1
    }

    pub fn last_le(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0i32;
        let mut high = (nums.len() - 1) as i32;
        let l = nums.len() as i32;
        while low <= high {
            let mid = (low + high) / 2;
            let mid_num = nums[mid as usize];
            // println!("low={}, mid={}, high={}, mid_num={}, target={}", low, mid, high, mid_num, target);
            if mid_num <= target {
                if mid == l - 1 || target < nums[(mid+1) as usize] {
                    return mid;
                }
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        -1
    }

    pub fn max_heapify(nums: &mut Vec<i32>, max_len: usize, start_pos: usize) {
        let left_pos = 2 * start_pos + 1;
        let right_pos = 2 * (start_pos + 1);

        let mut large_pos = None;
        let mut large_val = nums[start_pos];
        if left_pos < max_len && large_val < nums[left_pos] {
            large_val = nums[left_pos];
            large_pos = Some(left_pos);
        }

        if right_pos < max_len && large_val < nums[right_pos] {
            large_val = nums[right_pos];
            large_pos = Some(right_pos);
        }

        if let Some(large_pos) = large_pos {
            nums.swap(start_pos, large_pos);
            Self::max_heapify(nums, max_len, large_pos);
        }
    }

    pub fn heap_sort(nums: &mut Vec<i32>) {
        let n = nums.len();
        for i in (0..(n/2)).rev() {
            Self::max_heapify(nums, nums.len() , i);
        }

        for i in (0..n).rev() {
            nums.swap(0, i);
            Self::max_heapify(nums, i, 0);
        }
    }

    pub fn zero_right_digits(x : i32, digit_count: usize) -> i32 {
        x & (!0 << digit_count)
    }
    
    pub fn nth_digit(x : i32, n : usize) -> i32 {
        (x >> n) & 1
    }
    // if n-th digit is 1, return 2^n, else 0. 
    // n starts from 0. 
    pub fn nth_digit_power(x : i32, n : usize) -> i32 {
        x & (1 << n)
    }

    pub fn set_nth_digit_one(x: i32, n: usize) -> i32 {
        x | (1 << n)
    }

    pub fn set_nth_digit_zero(x: i32, n: usize) -> i32 {
        x & (!(1 << n))
    }

    // inclusive of nth
    pub fn set_zero_from_nth(x: i32, n: usize) -> i32  {
        x & ((1 << n) - 1)
    }

    // inclusive of nth
    pub fn set_zero_to_nth(x: i32, n: usize) -> i32  {
        x & (!((1 << (n + 1)) - 1))
    }

    // set the least significant bit-1 to 0. 
    pub fn set_lsb1_zero(x: i32) -> i32 {
        x & (x-1)
    }

    // 2^(pos of lsb 1)
    pub fn lsb1_power(x: i32) -> i32 {
        x & -x
    }

    pub fn digit1_count(mut x: i32) -> usize {
        let mut count = 0;
        while x != 0 {
            x = Self::set_lsb1_zero(x);
            count+=1;
        }
        count
    }

    // the largest power of 2 le to x
    pub fn largest_power(mut x : u32) -> u32 {
        // println!("{:#032b}", x);
        x = x | (x>>1);
        // println!("{:#032b}", x);
        x = x | (x>>2);
        // println!("{:#032b}", x);
        x = x | (x>>4);
        // println!("{:#032b}", x);
        x = x | (x>>8);
        // println!("{:#032b}", x);
        x = x | (x>>16);
        // println!("{:#032b}", x);
        x = (x+1)>>1;
        // println!("{:#032b}", x);
        x
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {

        assert_eq!(Solution::largest_power(9), 8);
        // assert_eq!(Solution::zero_right_digits(7, 1), 6);
        // assert_eq!(Solution::zero_right_digits(8, 2), 8);
        // assert_eq!(Solution::zero_right_digits(-3, 2), -4);

        // assert_eq!(Solution::nth_digit(-3, 0), 1);
        // assert_eq!(Solution::nth_digit(-3, 1), 0);

        // assert_eq!(Solution::nth_digit_power(-3, 1), 0);
        // assert_eq!(Solution::nth_digit_power(5, 2), 4);

        // assert_eq!(Solution::set_nth_digit_zero(5, 2), 1);
        // assert_eq!(Solution::set_nth_digit_one(5, 1), 7);

        // assert_eq!(Solution::set_zero_from_nth(5, 1), 1);
        // assert_eq!(Solution::set_zero_from_nth(5, 3), 5);
        // assert_eq!(Solution::set_zero_from_nth(-3, 3), 5);
        // assert_eq!(Solution::set_zero_from_nth(-3, 4), 13);

        // assert_eq!(Solution::set_zero_to_nth(5, 0), 4);
        // assert_eq!(Solution::set_zero_to_nth(5, 3), 0);

        // assert_eq!(Solution::set_zero_to_nth(-3, 2), -8);
        // assert_eq!(Solution::set_lsb1_zero(5), 4);
        // assert_eq!(Solution::set_lsb1_zero(6), 4);

        // assert_eq!(Solution::lsb1_power(6), 2);
        // assert_eq!(Solution::lsb1_power(7), 1);
        // assert_eq!(Solution::lsb1_power(8), 8);

        // let mut r = vec![12,11,13,5,6,7];
        // Solution::heap_sort(&mut r);
        // assert_eq!(r, vec![5,6,7,11,12,13]);
        // assert_eq!(Solution::first_equal(vec![1,1,3,3,5,5],0), -1);
        // assert_eq!(Solution::first_equal(vec![1,1,3,3,5,5],1), 0);
        // assert_eq!(Solution::first_equal(vec![1,1,3,3,5,5],2), -1);
        // assert_eq!(Solution::first_equal(vec![1,1,3,3,5,5],3), 2);
        // assert_eq!(Solution::first_equal(vec![1,1,3,3,5,5],5), 4);
        // assert_eq!(Solution::first_equal(vec![1,1,3,3,5,5],7), -1);

        // assert_eq!(Solution::last_equal(vec![1,1,3,3,5,5],0), -1);
        // assert_eq!(Solution::last_equal(vec![1,1,3,3,5,5],1), 1);
        // assert_eq!(Solution::last_equal(vec![1,1,3,3,5,5],2), -1);
        // assert_eq!(Solution::last_equal(vec![1,1,3,3,5,5],3), 3);
        // assert_eq!(Solution::last_equal(vec![1,1,3,3,5,5],5), 5);
        // assert_eq!(Solution::last_equal(vec![1,1,3,3,5,5],7), -1);

        // assert_eq!(Solution::first_gt(vec![1,1,3,3,5,5],0), 0);
        // assert_eq!(Solution::first_gt(vec![1,1,3,3,5,5],1), 2);
        // assert_eq!(Solution::first_gt(vec![1,1,3,3,5,5],2), 2);
        // assert_eq!(Solution::first_gt(vec![1,1,3,3,5,5],3), 4);
        // assert_eq!(Solution::first_gt(vec![1,1,3,3,5,5],5), -1);
        // assert_eq!(Solution::first_gt(vec![1,1,3,3,5,5],7), -1);

        // assert_eq!(Solution::first_ge(vec![1,1,3,3,5,5],0), 0);
        // assert_eq!(Solution::first_ge(vec![1,1,3,3,5,5],1), 0);
        // assert_eq!(Solution::first_ge(vec![1,1,3,3,5,5],2), 2);
        // assert_eq!(Solution::first_ge(vec![1,1,3,3,5,5],3), 2);
        // assert_eq!(Solution::first_ge(vec![1,1,3,3,5,5],5), 4);
        // assert_eq!(Solution::first_ge(vec![1,1,3,3,5,5],7), -1);

        // assert_eq!(Solution::last_lt(vec![1,1,3,3,5,5],0), -1);
        // assert_eq!(Solution::last_lt(vec![1,1,3,3,5,5],1), -1);
        // assert_eq!(Solution::last_lt(vec![1,1,3,3,5,5],2), 1);
        // assert_eq!(Solution::last_lt(vec![1,1,3,3,5,5],3), 1);
        // assert_eq!(Solution::last_lt(vec![1,1,3,3,5,5],5), 3);
        // assert_eq!(Solution::last_lt(vec![1,1,3,3,5,5],7), 5);

        // assert_eq!(Solution::last_le(vec![1,1,3,3,5,5],0), -1);
        // assert_eq!(Solution::last_le(vec![1,1,3,3,5,5],1), 1);
        // assert_eq!(Solution::last_le(vec![1,1,3,3,5,5],2), 1);
        // assert_eq!(Solution::last_le(vec![1,1,3,3,5,5],3), 3);
        // assert_eq!(Solution::last_le(vec![1,1,3,3,5,5],5), 5);
        // assert_eq!(Solution::last_le(vec![1,1,3,3,5,5],7), 5);
    }
}
