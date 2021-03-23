/**
 * [96] Unique Binary Search Trees
 *
 * Given an integer n, return the number of structurally unique BST's (binary search trees) which has exactly n nodes of unique values from 1 to n.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/uniquebstn3.jpg" style="width: 600px; height: 148px;" />
 * Input: n = 3
 * Output: 5
 * 
 * Example 2:
 * 
 * Input: n = 1
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 19
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-binary-search-trees/
// discuss: https://leetcode.com/problems/unique-binary-search-trees/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut result = vec![];
        
        result.push(1);
        result.push(1);
        for i in 2..=n {
            let mut sum = 0;
            for j in 0..i {
                let jj = i-1-j;
                sum += result.get(j as usize).unwrap() * result.get(jj as usize).unwrap();
            }
            result.push(sum);
        }
        result[n as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_96() {
        assert_eq!(Solution::num_trees(2), 2);
        assert_eq!(Solution::num_trees(3), 5);

    }
}
