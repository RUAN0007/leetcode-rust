/**
 * [223] Rectangle Area
 *
 * Given the coordinates of two rectilinear rectangles in a 2D plane, return the total area covered by the two rectangles.
 * The first rectangle is defined by its bottom-left corner (A, B) and its top-right corner (C, D).
 * The second rectangle is defined by its bottom-left corner (E, F) and its top-right corner (G, H).
 *  
 * Example 1:
 * <img alt="Rectangle Area" src="https://assets.leetcode.com/uploads/2021/03/16/rectangle-plane.jpg" style="width: 600px; height: 325px;" />
 * Input: A = -3, B = 0, C = 3, D = 4, E = 0, F = -1, G = 9, H = 2
 * Output: 45
 * 
 * Example 2:
 * 
 * Input: A = -2, B = -2, C = 2, D = 2, E = -2, F = -2, G = 2, H = 2
 * Output: 16
 * 
 *  
 * Constraints:
 * 
 * 	-10^4 <= A, B, C, D, E, F, G, H <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rectangle-area/
// discuss: https://leetcode.com/problems/rectangle-area/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn overlap(i1: (i32, i32), i2: (i32, i32)) -> i32 {
        let max_start = std::cmp::max(i1.0, i2.0);
        let min_end = std::cmp::min(i1.1, i2.1);
        if max_start < min_end {
            min_end - max_start
        } else {
            0
        }
    }

    pub fn area(lower_left: (i32, i32), upper_right: (i32, i32)) -> i32 {
        (upper_right.0 - lower_left.0) * (upper_right.1 - lower_left.1)
    }

    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        let overlap_x = Self::overlap((a, c), (e, g));
        let overlap_y = Self::overlap((b, d), (f, h));
        Self::area((a, b),(c, d)) + Self::area((e,f), (g,h)) - overlap_x * overlap_y
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_223() {
        assert_eq!(Solution::compute_area(0, 0, 0, 0, 0, 0, 0, 0), 0);
        assert_eq!(Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
        assert_eq!(Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
        assert_eq!(Solution::compute_area(-2, -2, 2, 2, -1, 4, 1, 6), 20);
    }
}
