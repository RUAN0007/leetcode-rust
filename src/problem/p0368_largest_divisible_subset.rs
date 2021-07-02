/**
 * [368] Largest Divisible Subset
 *
 * Given a set of distinct positive integers nums, return the largest subset answer such that every pair (answer[i], answer[j]) of elements in this subset satisfies:
 * 
 * 	answer[i] % answer[j] == 0, or
 * 	answer[j] % answer[i] == 0
 * 
 * If there are multiple solutions, return any of them.
 *  
 * Example 1:
 * 
 * Input: nums = [1,2,3]
 * Output: [1,2]
 * Explanation: [1,3] is also accepted.
 * 
 * Example 2:
 * 
 * Input: nums = [1,2,4,8]
 * Output: [1,2,4,8]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 1000
 * 	1 <= nums[i] <= 2 * 10^9
 * 	All the integers in nums are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-divisible-subset/
// discuss: https://leetcode.com/problems/largest-divisible-subset/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort(); 
        const NONE : Option<usize> = None;
        let l : usize = nums.len();
        // the size of the largest divisible subset of nums[0..=i], which must include nums[i]
        let mut subset_sizes : Vec<usize> = vec![1;l];
        let mut lasts : Vec<Option<usize>> = vec![NONE;l];

        let mut largest_pos : usize = 0;
        let mut largest_size : usize = 0;

        for (i, &num) in nums.iter().enumerate() {
            let mut prev_largest_size : usize = 0;
            let mut prev_largest_pos : Option<usize> = None;
            for j in 0..i {
                if num % nums[j] == 0 && subset_sizes[j] > prev_largest_size {
                    prev_largest_size = subset_sizes[j];
                    prev_largest_pos = Some(j);
                }
            }
            subset_sizes[i] = prev_largest_size + 1;
            lasts[i] = prev_largest_pos;

            if subset_sizes[i] > largest_size {
                largest_size = subset_sizes[i];
                largest_pos = i;
            }
        }

        let mut r : Vec<i32> = vec![nums[largest_pos]];
        let mut cur_pos : usize = largest_pos;
        while let Some(next_pos) = lasts[cur_pos] {
            r.push(nums[next_pos]);
            cur_pos = next_pos;
        }

        r.into_iter().rev().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_368() {
        assert_eq!(Solution::largest_divisible_subset(vec![1,2,3]), vec![1,2]);
        assert_eq!(Solution::largest_divisible_subset(vec![1,2,4,8]), vec![1,2,4,8]);
        assert_eq!(Solution::largest_divisible_subset(vec![1,2,4,6,8]), vec![1,2,4,8]);
    }
}
