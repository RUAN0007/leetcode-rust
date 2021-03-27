/**
 * [47] Permutations II
 *
 * Given a collection of numbers, nums, that might contain duplicates, return all possible unique permutations in any order.
 *  
 * Example 1:
 * 
 * Input: nums = [1,1,2]
 * Output:
 * [[1,1,2],
 *  [1,2,1],
 *  [2,1,1]]
 * 
 * Example 2:
 * 
 * Input: nums = [1,2,3]
 * Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 8
 * 	-10 <= nums[i] <= 10
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/permutations-ii/
// discuss: https://leetcode.com/problems/permutations-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::{collections::HashSet, mem::swap};

impl Solution {
    pub fn helper(nums: &mut Vec<i32>, start_pos : usize, end_pos : usize) -> Vec<Vec<i32>> {
        if start_pos == end_pos {
            return vec![vec![]];
        }
        let mut my_perms : Vec<Vec<i32>> = vec![];
        let mut swapped = HashSet::new();
        for i in start_pos..end_pos {
            if swapped.contains(&nums[i]) {
                continue;
            }
            swapped.insert(nums[i]);

            nums.swap(start_pos, i);
            let mut prev_perms = Self::helper(nums, start_pos+1, end_pos);

            for prev_perm in &mut prev_perms {
                let mut my_perm = vec![nums[start_pos]];
                my_perm.append(prev_perm);
                // println!("\t myperm: {:?}", my_perm);
                my_perms.push(my_perm);
            }

            nums.swap(start_pos, i);
        }
        // println!("nums: {:?}, start_pos: {}, perm: {:?}", nums, start_pos, my_perms);
        // println!("");
        my_perms
    }

    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let l = nums.len();
        Self::helper(&mut nums, 0, l)
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_47() {
        assert_eq!(
            Solution::permute(vec![1, 1, 2]),
            vec![vec![2, 1, 1], vec![1, 2, 1], vec![1, 1, 2],]
        );
        assert_eq!(Solution::permute(vec![1, 1, 1]), vec![vec![1, 1, 1],]);
        assert_eq!(
            Solution::permute(vec![1, 1, 1, 2]),
            vec![
                vec![2, 1, 1, 1],
                vec![1, 2, 1, 1],
                vec![1, 1, 2, 1],
                vec![1, 1, 1, 2],
            ]
        );
        assert_eq!(
            Solution::permute(vec![1, 1, 2, 2, 3, 3]),
            vec![
                vec![3, 3, 2, 2, 1, 1],
                vec![3, 2, 3, 2, 1, 1],
                vec![2, 3, 3, 2, 1, 1],
                vec![3, 2, 2, 3, 1, 1],
                vec![2, 3, 2, 3, 1, 1],
                vec![2, 2, 3, 3, 1, 1],
                vec![3, 3, 2, 1, 2, 1],
                vec![3, 2, 3, 1, 2, 1],
                vec![2, 3, 3, 1, 2, 1],
                vec![3, 3, 1, 2, 2, 1],
                vec![3, 1, 3, 2, 2, 1],
                vec![1, 3, 3, 2, 2, 1],
                vec![3, 2, 1, 3, 2, 1],
                vec![2, 3, 1, 3, 2, 1],
                vec![3, 1, 2, 3, 2, 1],
                vec![1, 3, 2, 3, 2, 1],
                vec![2, 1, 3, 3, 2, 1],
                vec![1, 2, 3, 3, 2, 1],
                vec![3, 2, 2, 1, 3, 1],
                vec![2, 3, 2, 1, 3, 1],
                vec![2, 2, 3, 1, 3, 1],
                vec![3, 2, 1, 2, 3, 1],
                vec![2, 3, 1, 2, 3, 1],
                vec![3, 1, 2, 2, 3, 1],
                vec![1, 3, 2, 2, 3, 1],
                vec![2, 1, 3, 2, 3, 1],
                vec![1, 2, 3, 2, 3, 1],
                vec![2, 2, 1, 3, 3, 1],
                vec![2, 1, 2, 3, 3, 1],
                vec![1, 2, 2, 3, 3, 1],
                vec![3, 3, 2, 1, 1, 2],
                vec![3, 2, 3, 1, 1, 2],
                vec![2, 3, 3, 1, 1, 2],
                vec![3, 3, 1, 2, 1, 2],
                vec![3, 1, 3, 2, 1, 2],
                vec![1, 3, 3, 2, 1, 2],
                vec![3, 2, 1, 3, 1, 2],
                vec![2, 3, 1, 3, 1, 2],
                vec![3, 1, 2, 3, 1, 2],
                vec![1, 3, 2, 3, 1, 2],
                vec![2, 1, 3, 3, 1, 2],
                vec![1, 2, 3, 3, 1, 2],
                vec![3, 3, 1, 1, 2, 2],
                vec![3, 1, 3, 1, 2, 2],
                vec![1, 3, 3, 1, 2, 2],
                vec![3, 1, 1, 3, 2, 2],
                vec![1, 3, 1, 3, 2, 2],
                vec![1, 1, 3, 3, 2, 2],
                vec![3, 2, 1, 1, 3, 2],
                vec![2, 3, 1, 1, 3, 2],
                vec![3, 1, 2, 1, 3, 2],
                vec![1, 3, 2, 1, 3, 2],
                vec![2, 1, 3, 1, 3, 2],
                vec![1, 2, 3, 1, 3, 2],
                vec![3, 1, 1, 2, 3, 2],
                vec![1, 3, 1, 2, 3, 2],
                vec![1, 1, 3, 2, 3, 2],
                vec![2, 1, 1, 3, 3, 2],
                vec![1, 2, 1, 3, 3, 2],
                vec![1, 1, 2, 3, 3, 2],
                vec![3, 2, 2, 1, 1, 3],
                vec![2, 3, 2, 1, 1, 3],
                vec![2, 2, 3, 1, 1, 3],
                vec![3, 2, 1, 2, 1, 3],
                vec![2, 3, 1, 2, 1, 3],
                vec![3, 1, 2, 2, 1, 3],
                vec![1, 3, 2, 2, 1, 3],
                vec![2, 1, 3, 2, 1, 3],
                vec![1, 2, 3, 2, 1, 3],
                vec![2, 2, 1, 3, 1, 3],
                vec![2, 1, 2, 3, 1, 3],
                vec![1, 2, 2, 3, 1, 3],
                vec![3, 2, 1, 1, 2, 3],
                vec![2, 3, 1, 1, 2, 3],
                vec![3, 1, 2, 1, 2, 3],
                vec![1, 3, 2, 1, 2, 3],
                vec![2, 1, 3, 1, 2, 3],
                vec![1, 2, 3, 1, 2, 3],
                vec![3, 1, 1, 2, 2, 3],
                vec![1, 3, 1, 2, 2, 3],
                vec![1, 1, 3, 2, 2, 3],
                vec![2, 1, 1, 3, 2, 3],
                vec![1, 2, 1, 3, 2, 3],
                vec![1, 1, 2, 3, 2, 3],
                vec![2, 2, 1, 1, 3, 3],
                vec![2, 1, 2, 1, 3, 3],
                vec![1, 2, 2, 1, 3, 3],
                vec![2, 1, 1, 2, 3, 3],
                vec![1, 2, 1, 2, 3, 3],
                vec![1, 1, 2, 2, 3, 3]
            ]
        );
    }
}
