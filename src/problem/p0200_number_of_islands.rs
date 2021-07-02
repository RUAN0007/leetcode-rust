/**
 * [200] Number of Islands
 *
 * Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), return the number of islands.
 * An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.
 *  
 * Example 1:
 * 
 * Input: grid = [
 *   ["1","1","1","1","0"],
 *   ["1","1","0","1","0"],
 *   ["1","1","0","0","0"],
 *   ["0","0","0","0","0"]
 * ]
 * Output: 1
 * 
 * Example 2:
 * 
 * Input: grid = [
 *   ["1","1","0","0","0"],
 *   ["1","1","0","0","0"],
 *   ["0","0","1","0","0"],
 *   ["0","0","0","1","1"]
 * ]
 * Output: 3
 * 
 *  
 * Constraints:
 * 
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 300
 * 	grid[i][j] is '0' or '1'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-islands/
// discuss: https://leetcode.com/problems/number-of-islands/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn traverse_set_zero(grid : &mut Vec<Vec<char>>, i : i32, j : i32) {
        if 0 <= i && i < grid.len() as i32 && 0 <= j && j < grid[0].len() as i32 && grid[i as usize][j as usize] == '1' {
            grid[i as usize][j as usize] = '0';
            Self::traverse_set_zero(grid, i+1, j);
            Self::traverse_set_zero(grid, i-1, j);
            Self::traverse_set_zero(grid, i, j+1);
            Self::traverse_set_zero(grid, i, j-1);
        }
    }

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let row_count : usize = grid.len();
        let col_count : usize = grid[0].len();
        let mut count : i32 = 0;
        for i in 0..row_count {
            for j in 0..col_count {
                if grid[i][j] == '1' {
                    Self::traverse_set_zero(&mut grid, i as i32, j as i32);
                    count += 1;
                }
            }
        }
        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_200() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0',],
                vec!['1', '1', '0', '1', '0',],
                vec!['1', '1', '0', '0', '0',],
                vec!['0', '0', '0', '0', '0',],
            ]),
            1
        );
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', 'o', '1', '0',],
                vec!['1', '1', '0', '1', '0',],
                vec!['1', '1', '0', '0', '0',],
                vec!['0', '0', '0', '1', '1',],
            ]),
            3
        );
    }
}
