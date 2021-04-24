use std::usize;

/**
 * [416] Partition Equal Subset Sum
 *
 * Given a non-empty array nums containing only positive integers, find if the array can be partitioned into two subsets such that the sum of elements in both subsets is equal.
 *  
 * Example 1:
 * 
 * Input: nums = [1,5,11,5]
 * Output: true
 * Explanation: The array can be partitioned as [1, 5, 5] and [11].
 * 
 * Example 2:
 * 
 * Input: nums = [1,2,3,5]
 * Output: false
 * Explanation: The array cannot be partitioned into equal sum subsets.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 200
 * 	1 <= nums[i] <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-equal-subset-sum/
// discuss: https://leetcode.com/problems/partition-equal-subset-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum = 0;
        for &num in &nums {
            sum += num;
        }
        if (sum % 2 == 1) {return false; }
        let target = (sum / 2) as usize;
        let num_count = nums.len();
        let mut result = vec![vec![false;target+1];num_count + 1];
        // result[i][j] imply whether possible to use the subsets of first i elements to reach the target sum j. 

        result[0][0] = true;
        for i in 1..=num_count {
            result[i][0] = true;
            for j in 1..=target {
                let this_num = nums[i-1] as usize;
                if this_num <= j {
                    result[i][j] = result[i-1][j] || result[i-1][j-this_num];
                } else {
                    result[i][j] = result[i-1][j];
                }
            }
        }

        result[num_count][target]
    }
    // pub fn can_partition(nums: Vec<i32>) -> bool {
    //     let mut sum = 0;
    //     for &num in &nums {
    //         sum += num;
    //     }
    //     if (sum % 2 == 1) {return false; }

    //     let mut result = vec![false; (sum/2) as usize + 1];
    //     result[0] = true; 

        // after each iteration at i, result[j] indicates whether
        // there exists a subset from the first i elements that sum to j
        // for &num in &nums {
        //     for s in (num..=sum/2).rev() {
                // Note: we iterate s in opposite order. 
                // This is because when updating result[s], 
                //   we wanna read result[s-num] updated in the previous iteration. 
                // The increasing order of s may update result[s-num] first and then updat results[s]. During the latter's update, the accessed result[s-num] no longer represents for the previous iteration. 
    //             let s= s as usize;
    //             let num = num as usize;
    //             result[s] = result[s] || result[s-num];
    //         }
    //     }

    //     result[(sum/2) as usize]
    // }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_416() {
        assert!(Solution::can_partition(vec![1,5,11,5]));
        assert!(!Solution::can_partition(vec![1,2,3,5]));
    }
}
