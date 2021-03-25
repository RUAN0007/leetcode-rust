use std::collections::HashMap;

/**
 * [494] Target Sum
 *
 * You are given a list of non-negative integers, a1, a2, ..., an, and a target, S. Now you have 2 symbols + and -. For each integer, you should choose one from + and - as its new symbol.
 * Find out how many ways to assign symbols to make sum of integers equal to target S.
 * Example 1:
 * 
 * Input: nums is [1, 1, 1, 1, 1], S is 3. 
 * Output: 5
 * Explanation: 
 * -1+1+1+1+1 = 3
 * +1-1+1+1+1 = 3
 * +1+1-1+1+1 = 3
 * +1+1+1-1+1 = 3
 * +1+1+1+1-1 = 3
 * There are 5 ways to assign symbols to make the sum of nums be target 3.
 * 
 *  
 * Constraints:
 * 
 * 	The length of the given array is positive and will not exceed 20.
 * 	The sum of elements in the given array will not exceed 1000.
 * 	Your output answer is guaranteed to be fitted in a 32-bit integer.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/target-sum/
// discuss: https://leetcode.com/problems/target-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn nums_subsets_sum(nums: Vec<i32>, target: i32) -> usize {
        let target = target as usize;
        let mut result = vec![0; target + 1];
        result[0] = 1; 

        for &num in &nums {
            for s in (num as usize ..=target).rev() {
                let num = num as usize;
                result[s] = result[s] + result[s-num];
            }
        }

        result[target]
    }

    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        let mut sum = 0;
        for &num in &nums {
            sum += num;
        }

        if sum < s || (s + sum) % 2 == 1 {
            0
        } else {
            Self::nums_subsets_sum(nums, (s+sum)/2) as i32
        }
 
    }


    // pub fn find_target_sum_ways_2(nums: Vec<i32>, s: i32) -> i32 {
    //     let mut sum = 0;
    //     for &num in &nums {
    //         sum += num;
    //     }

    //     if s < -sum || sum < s {
    //         return 0;
    //     }
    //     // ways[k]{s->n} denotes that there are n ways to sum up to s with the first k coins. 
    //     let mut ways: Vec<HashMap<i32, usize>> = vec![HashMap::new(); nums.len()+1]; 

    //     ways[0].insert(0, 1);  // there is a way to sum to 0 without any elements. 
    //     for (i, &num) in nums.iter().enumerate() {
    //         let i = i + 1;
    //         for j in -sum..=sum {
    //             let plus_sum = j - num; // assume a plus sign before num to attain j. 

    //             let mut plus_ways = 0usize;
    //             if let Some(&p) = ways[i-1].get(&plus_sum) {
    //                 plus_ways = p;
    //             }


    //             let minus_sum = j + num; // assume a minus sign before num to attain j. 

    //             let mut minus_ways = 0usize;
    //             if let Some(&p) = ways[i-1].get(&minus_sum) {
    //                 minus_ways = p;
    //             }
    //             ways[i].insert(j, minus_ways + plus_ways);
    //         }
    //     }
    //     if let Some(&r) = ways[nums.len()].get(&s) {
    //         r as i32
    //     } else {
    //         0
    //     }
    // }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_494() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }
}
