/**
 * [78] Subsets
 *
 * Given an integer array nums of unique elements, return all possible subsets (the power set).
 * The solution set must not contain duplicate subsets. Return the solution in any order.
 *  
 * Example 1:
 * 
 * Input: nums = [1,2,3]
 * Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
 * 
 * Example 2:
 * 
 * Input: nums = [0]
 * Output: [[],[0]]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10
 * 	-10 <= nums[i] <= 10
 * 	All the numbers of nums are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/subsets/
// discuss: https://leetcode.com/problems/subsets/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here


impl Solution {
    pub fn helper(nums: &Vec<i32>, pos: usize) -> Vec<Vec<i32>> {
        // println!("=========pos: {}================", pos);
        let mut results = vec![vec![]];
        for i in pos..nums.len() {

            let mut prev_subsets = Self::helper(nums, i+1);
            // println!("   pos: {}, i: {}, prev_subsets: {:?}", pos, i, prev_subsets);
            for mut prev_subset in prev_subsets {
                let mut my_subset = vec![nums[i]];
                my_subset.append(&mut prev_subset);
                results.push(my_subset);
            }
        }
        // println!("pos: {}, subsets: {:?}", pos, results);
        // println!("=========END pos: {}================", pos);
        results
    }

    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::helper(&nums, 0)
    }

}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_78() {
        assert_eq!(Solution::subsets(vec![]), vec![vec![]]);
        assert_eq!(Solution::subsets(vec![1]), vec![vec![], vec![1]]);
        assert_eq!(
            Solution::subsets(vec![1, 2]),
            vec![vec![], vec![2], vec![1], vec![1, 2]]
        );
    }
}
