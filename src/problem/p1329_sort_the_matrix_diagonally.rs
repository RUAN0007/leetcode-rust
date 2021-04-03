/**
 * [1329] Sort the Matrix Diagonally
 *
 * A matrix diagonal is a diagonal line of cells starting from some cell in either the topmost row or leftmost column and going in the bottom-right direction until reaching the matrix's end. For example, the matrix diagonal starting from mat[2][0], where mat is a 6 x 3 matrix, includes cells mat[2][0], mat[3][1], and mat[4][2].
 * Given an m x n matrix mat of integers, sort each matrix diagonal in ascending order and return the resulting matrix.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/21/1482_example_1_2.png" style="width: 500px; height: 198px;" />
 * Input: mat = [[3,3,1,1],[2,2,1,2],[1,1,1,2]]
 * Output: [[1,1,1,1],[1,2,2,2],[1,2,3,3]]
 * 
 * Example 2:
 * 
 * Input: mat = [[11,25,66,1,69,7],[23,55,17,45,15,52],[75,31,36,44,58,8],[22,27,33,25,68,4],[84,28,14,11,5,50]]
 * Output: [[5,17,4,1,52,7],[11,11,25,45,8,69],[14,23,25,44,58,15],[22,27,31,36,50,66],[84,28,75,33,55,68]]
 * 
 *  
 * Constraints:
 * 
 * 	m == mat.length
 * 	n == mat[i].length
 * 	1 <= m, n <= 100
 * 	1 <= mat[i][j] <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-the-matrix-diagonally/
// discuss: https://leetcode.com/problems/sort-the-matrix-diagonally/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort(mat: &mut Vec<Vec<i32>>, ptrs: Vec<(usize, usize)>) {
        let mut nums = Vec::new();
        for &ptr in &ptrs {
            nums.push(mat[ptr.0][ptr.1]);
        }
        nums.sort();
        let mut i = 0;
        for &ptr in &ptrs {
            mat[ptr.0][ptr.1] = nums[i];
            i+=1;
        }
    }

    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut start_pts = vec![];
        for i in 0..mat.len() {
            start_pts.push((i, 0));
        }
        for j in 1..mat[0].len() {
            start_pts.push((0, j));
        }

        for start_pt in start_pts {
            let mut i = start_pt.0;
            let mut j = start_pt.1;
            let mut diag = vec![];
            while i < mat.len() && j < mat[0].len() {
                diag.push((i,j));
                i+=1;
                j+=1;
            }
            Self::sort(&mut mat, diag);
        }
        mat
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1329() {
    }
}
