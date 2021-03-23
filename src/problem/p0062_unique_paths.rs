/**
 * [62] Unique Paths
 *
 * A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
 * The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
 * How many possible unique paths are there?
 *  
 * Example 1:
 * <img src="https://assets.leetcode.com/uploads/2018/10/22/robot_maze.png" style="width: 400px; height: 183px;" />
 * Input: m = 3, n = 7
 * Output: 28
 * 
 * Example 2:
 * 
 * Input: m = 3, n = 2
 * Output: 3
 * Explanation:
 * From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
 * 1. Right -> Down -> Down
 * 2. Down -> Down -> Right
 * 3. Down -> Right -> Down
 * 
 * Example 3:
 * 
 * Input: m = 7, n = 3
 * Output: 28
 * 
 * Example 4:
 * 
 * Input: m = 3, n = 3
 * Output: 6
 * 
 *  
 * Constraints:
 * 
 * 	1 <= m, n <= 100
 * 	It's guaranteed that the answer will be less than or equal to 2 * 10^9.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-paths/
// discuss: https://leetcode.com/problems/unique-paths/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // m : depth
        // n : width
        let mut v: Vec<Vec<i32>> = (0..m)
        .map(|_| 
            (0..n)
            .map(|_| 0)
            .collect()
        ).collect();
        // println!("depth = {}, width = {}, m = {}, n = {}", v.len(), v[0].len(), m, n);

        for i  in 0..m {
            for j  in 0..n {
                let i = i as usize;
                let j = j as usize;
                if i == 0 || j == 0 {
                    println!("i = {}, j = {}", i, j) ;
                    v[i][j] = 1;
                } else {
                    v[i][j] = v[i-1][j] + v[i][j-1];
                }
            }
        }

        v[(m-1) as usize][(n-1) as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_62() {
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(1, 1), 1);
        assert_eq!(Solution::unique_paths(2, 2), 2);
        assert_eq!(Solution::unique_paths(36, 7), 4496388);
    }
}
