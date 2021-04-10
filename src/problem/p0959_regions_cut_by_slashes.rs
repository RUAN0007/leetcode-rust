/**
 * [959] Regions Cut By Slashes
 *
 * In a N x N grid composed of 1 x 1 squares, each 1 x 1 square consists of a /, \, or blank space.  These characters divide the square into contiguous regions.
 * 
 * (Note that backslash characters are escaped, so a \ is represented as "\\".)
 * 
 * Return the number of regions.
 * 
 *  
 * 
 * <div>
 * <div>
 * <div>
 * <div>
 * <div>
 * <ol>
 * </ol>
 * </div>
 * </div>
 * </div>
 * </div>
 * </div>
 * 
 * <div>
 * Example 1:
 * 
 * 
 * Input:
 * <span id="example-input-1-1">[
 *   " /",
 *   "/ "
 * ]</span>
 * Output: <span id="example-output-1">2</span>
 * Explanation: The 2x2 grid is as follows:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/1.png" style="width: 82px; height: 82px;" />
 * 
 * 
 * <div>
 * Example 2:
 * 
 * 
 * Input:
 * <span id="example-input-2-1">[
 *   " /",
 *   "  "
 * ]</span>
 * Output: <span id="example-output-2">1</span>
 * Explanation: The 2x2 grid is as follows:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/2.png" style="width: 82px; height: 82px;" />
 * 
 * 
 * <div>
 * Example 3:
 * 
 * 
 * Input:
 * <span id="example-input-3-1">[
 *   "\\/",
 *   "/\\"
 * ]</span>
 * Output: <span id="example-output-3">4</span>
 * Explanation: (Recall that because \ characters are escaped, "\\/" refers to \/, and "/\\" refers to /\.)
 * The 2x2 grid is as follows:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/3.png" style="width: 82px; height: 82px;" />
 * 
 * 
 * <div>
 * Example 4:
 * 
 * 
 * Input:
 * <span id="example-input-4-1">[
 *   "/\\",
 *   "\\/"
 * ]</span>
 * Output: <span id="example-output-4">5</span>
 * Explanation: (Recall that because \ characters are escaped, "/\\" refers to /\, and "\\/" refers to \/.)
 * The 2x2 grid is as follows:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/4.png" style="width: 82px; height: 82px;" />
 * 
 * 
 * <div>
 * Example 5:
 * 
 * 
 * Input:
 * <span id="example-input-5-1">[
 *   "//",
 *   "/ "
 * ]</span>
 * Output: <span id="example-output-5">3</span>
 * Explanation: The 2x2 grid is as follows:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/5.png" style="width: 82px; height: 82px;" />
 * 
 * 
 *  
 * 
 * Note:
 * 
 * <ol>
 * 	1 <= grid.length == grid[0].length <= 30
 * 	grid[i][j] is either '/', '\', or ' '.
 * </ol>
 * </div>
 * </div>
 * </div>
 * </div>
 * </div>
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/regions-cut-by-slashes/
// discuss: https://leetcode.com/problems/regions-cut-by-slashes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn dfs(ggrid: &mut Vec<Vec<i32>>, i : i32, j : i32) {
        let n = ggrid.len() as i32;
        if 0 <= i && i < n && 0 <= j && j < n && ggrid[i as usize][j as usize] == 0{
            ggrid[i as usize][j as usize] = 1;
            Self::dfs(ggrid, i+1,j);
            Self::dfs(ggrid, i-1,j);
            Self::dfs(ggrid, i,j+1);
            Self::dfs(ggrid, i,j-1);
        }
    }

    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let n = grid.len();
        let mut ggrid = vec![vec![0;3*n];3*n];
        for i in 0..n {
            // transform to vec for constant reference time complexity
            let row : Vec<char> = grid[i].chars().collect(); 
            for j in 0..n {
                if row[j] == '/' {
                    ggrid[3*i][3*j+2]=1;
                    ggrid[3*i+1][3*j+1]=1;
                    ggrid[3*i+2][3*j]=1;
                } else if  row[j] == '\\' {
                    ggrid[3*i][3*j]=1;
                    ggrid[3*i+1][3*j+1]=1;
                    ggrid[3*i+2][3*j+2]=1;
                } else {
                    //do nothing
                }
            }
        }
        let mut count = 0i32;
        for i in 0..3*n as i32 {
            // transform to vec for constant reference time complexity
            for j in 0..3*n as i32{
                if ggrid[i as usize][j as usize] == 0 {
                    Self::dfs(&mut ggrid, i, j);
                    count+=1;
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
    fn test_959() {
        assert_eq!(Solution::regions_by_slashes(vec![" /".to_owned(),"/ ".to_owned()]), 2);
    }
}
