
/**
 * [15] 3Sum
 *
 * Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0? Find all unique triplets in the array which gives the sum of zero.
 * Notice that the solution set must not contain duplicate triplets.
 *  
 * Example 1:
 * Input: nums = [-1,0,1,2,-1,-4]
 * Output: [[-1,-1,2],[-1,0,1]]
 * Example 2:
 * Input: nums = []
 * Output: []
 * Example 3:
 * Input: nums = [0]
 * Output: []
 *  
 * Constraints:
 * 
 * 	0 <= nums.length <= 3000
 * 	-10^5 <= nums[i] <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/3sum/
// discuss: https://leetcode.com/problems/3sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut num2idx = HashMap::new();

        for (idx, num) in nums.iter().enumerate() {
            let diff = 0 - num;
            num2idx.insert(num, idx);
        }


        let mut result = vec![];
        for (i, &num1) in nums.iter().enumerate() {
            if 0 < i && nums[i-1] == num1 {continue; }

            for j in (i+1..nums.len()) {
                if i+1 < j && nums[j-1] == nums[j] {continue; }
                let diff = 0 - num1 - nums[j]; 
                if let Some(&idx) = num2idx.get(&diff) {
                    if j < idx {
                        result.push(vec![nums[i], nums[j], nums[idx]]);
                    }
                }
            } 
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_15() {
    }
}
