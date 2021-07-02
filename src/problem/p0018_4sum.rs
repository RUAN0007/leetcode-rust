/**
 * [18] 4Sum
 *
 * Given an array nums of n integers, return an array of all the unique quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:
 * 
 * 	0 <= a, b, c, d < n
 * 	a, b, c, and d are distinct.
 * 	nums[a] + nums[b] + nums[c] + nums[d] == target
 * 
 * You may return the answer in any order.
 *  
 * Example 1:
 * 
 * Input: nums = [1,0,-1,0,-2,2], target = 0
 * Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
 * 
 * Example 2:
 * 
 * Input: nums = [2,2,2,2,2], target = 8
 * Output: [[2,2,2,2]]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 200
 * 	-10^9 <= nums[i] <= 10^9
 * 	-10^9 <= target <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/4sum/
// discuss: https://leetcode.com/problems/4sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() <= 3 {return vec![];}
        nums.sort();
        let mut result : Vec<Vec<i32>> = vec![];
        let mut num2pos : HashMap<i32, usize> = HashMap::new();

        for i in 0..nums.len() {
            num2pos.insert(nums[i], i); // the last pos wins.
        }

        let mut i : usize = 0;
        let mut result : Vec<Vec<i32>> = vec![];
        while i < nums.len() {
            while i != 0 && i < nums.len() && nums[i-1] == nums[i] {
                i+=1;
            }
            if i == nums.len() {break}

            let num1 : i32 = nums[i];
            let mut j : usize = i + 1;
            while j < nums.len() {
                while j != i+1 && j < nums.len() && nums[j-1] == nums[j] {
                    j+=1;
                }
                if j == nums.len() {break}
                let num2 : i32 = nums[j];
                let mut k : usize = j + 1;

                while k < nums.len() {
                    while k != j+1 && k < nums.len() && nums[k-1] == nums[k] {
                        k+=1;
                    }
                    if k == nums.len() {break}
                    let num3 : i32 = nums[k];

                    let num4 : i32 = target - num1 - num2 - num3;
                    if let Some(&num4_pos) = num2pos.get(&num4) {
                       if num4_pos > k {
                          result.push(vec![num1, num2, num3, num4]);
                       }
                    }
                    k +=1;
                }
                j+=1;
            }
            i+=1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_18() {
        assert_eq!(Solution::four_sum(vec![1,0,-1,0,-2,2], 0), vec![vec![-2,-1,1,2],vec![-2,0,0,2],vec![-1,0,0,1]]);

        assert_eq!(Solution::four_sum(vec![2,2,2,2,2], 8), vec![vec![2,2,2,2]]);
    }
}
