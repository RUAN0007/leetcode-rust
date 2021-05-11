/**
 * [312] Burst Balloons
 *
 * You are given n balloons, indexed from 0 to n - 1. Each balloon is painted with a number on it represented by an array nums. You are asked to burst all the balloons.
 * If you burst the i^th balloon, you will get nums[i - 1] * nums[i] * nums[i + 1] coins. If i - 1 or i + 1 goes out of bounds of the array, then treat it as if there is a balloon with a 1 painted on it.
 * Return the maximum coins you can collect by bursting the balloons wisely.
 *  
 * Example 1:
 * 
 * Input: nums = [3,1,5,8]
 * Output: 167
 * Explanation:
 * nums = [3,1,5,8] --> [3,5,8] --> [3,8] --> [8] --> []
 * coins =  3*1*5    +   3*5*8   +  1*3*8  + 1*8*1 = 167
 * Example 2:
 * 
 * Input: nums = [1,5]
 * Output: 10
 * 
 *  
 * Constraints:
 * 
 * 	n == nums.length
 * 	1 <= n <= 500
 * 	0 <= nums[i] <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/burst-balloons/
// discuss: https://leetcode.com/problems/burst-balloons/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn dp_memo(result : &mut Vec<Vec<i32>>, nums : &Vec<i32>, start : usize, end : usize) -> i32 {
        if start == end {return 0}
        if result[start][end] >= 0 {return result[start][end]}
        let mut ans : i32 = 0;
        for i in start..end {
            let left_max : i32 = Self::dp_memo(result, nums, start, i);
            let right_max : i32 = Self::dp_memo(result, nums, i+1, end);
            let burst_ptr : i32 = nums[start] * nums[i+1] * nums[end+1];
            // if start == 3 && end == 4 {
            //     println!("i={}, left_max={}, right_max={}, burst_ptr={}", i, left_max, right_max, burst_ptr);
            //     println!("i={}, nums[start]={}, nums[i+1]={}, nums[end]={}", i, nums[start], nums[i+1], nums[end]);
            // }
            ans = std::cmp::max(ans, left_max + burst_ptr + right_max);
        }
        result[start][end] = ans;
        ans
    }

    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n : usize = nums.len();
        let mut augmented : Vec<i32> = vec![1];
        for &num in nums.iter() {
            augmented.push(num);
        }
        augmented.push(1);

        let mut result : Vec<Vec<i32>> = vec![vec![-1;n+1];n+1];
        Self::dp_memo(&mut result, &augmented, 0usize, n);
        for r in result.iter() {
            println!("{:?}", r);
        }
        result[0][n]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_312() {
        assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
    }
}
