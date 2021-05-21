/**
 * [391] Perfect Rectangle
 *
 * Given an array rectangles where rectangles[i] = [xi, yi, ai, bi] represents an axis-aligned rectangle. The bottom-left point of the rectangle is (xi, yi) and the top-right point of it is (ai, bi).
 * Return true if all the rectangles together form an exact cover of a rectangular region.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/27/perectrec1-plane.jpg" style="width: 300px; height: 294px;" />
 * Input: rectangles = [[1,1,3,3],[3,1,4,2],[3,2,4,4],[1,3,2,4],[2,3,3,4]]
 * Output: true
 * Explanation: All 5 rectangles together form an exact cover of a rectangular region.
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/27/perfectrec2-plane.jpg" style="width: 300px; height: 294px;" />
 * Input: rectangles = [[1,1,2,3],[1,3,2,4],[3,1,4,2],[3,2,4,4]]
 * Output: false
 * Explanation: Because there is a gap between the two rectangular regions.
 * 
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/27/perfectrec3-plane.jpg" style="width: 300px; height: 294px;" />
 * Input: rectangles = [[1,1,3,3],[3,1,4,2],[1,3,2,4],[3,2,4,4]]
 * Output: false
 * Explanation: Because there is a gap in the top center.
 * 
 * Example 4:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/27/perfecrrec4-plane.jpg" style="width: 300px; height: 294px;" />
 * Input: rectangles = [[1,1,3,3],[3,1,4,2],[1,3,2,4],[2,2,4,4]]
 * Output: false
 * Explanation: Because two of the rectangles overlap with each other.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= rectangles.length <= 2 * 10^4
 * 	rectangles[i].length == 4
 * 	-10^5 <= xi, yi, ai, bi <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/perfect-rectangle/
// discuss: https://leetcode.com/problems/perfect-rectangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
const TOP_LEFT : usize = 0b1000;
const TOP_RIGHT : usize = 0b0100;
const BOTTOM_LEFT : usize = 0b0010;
const BOTTOM_RIGHT : usize = 0b0001;

use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut min_x : i32 = 10000;
        let mut min_y : i32 = 10000;
        let mut max_x : i32 = -10000;
        let mut max_y : i32 = -10000;

        let mut points : HashMap<(i32,i32), usize> = HashMap::new();
        for rect in rectangles {
            let this_bottom_right = points.entry((rect[2], rect[1])).or_insert(0usize); 
            if *this_bottom_right & BOTTOM_RIGHT == 0 {
                *this_bottom_right |= BOTTOM_RIGHT;
            } else {
                return false;
            }

            let this_bottom_left = points.entry((rect[0], rect[1])).or_insert(0usize); 
            if *this_bottom_left & BOTTOM_LEFT == 0 {
                *this_bottom_left |= BOTTOM_LEFT;
            } else {
                return false;
            }

            let this_top_left = points.entry((rect[0], rect[3])).or_insert(0usize); 
            if *this_top_left & TOP_LEFT == 0 {
                *this_top_left |= TOP_LEFT;
            } else {
                return false;
            }

            let this_top_right = points.entry((rect[2], rect[3])).or_insert(0usize); 
            if *this_top_right & TOP_RIGHT == 0 {
                *this_top_right |= TOP_RIGHT;
            } else {
                return false;
            }

            min_x = std::cmp::min(min_x, rect[0]);
            min_x = std::cmp::min(min_x, rect[2]);
            max_x = std::cmp::max(max_x, rect[0]);
            max_x = std::cmp::max(max_x, rect[2]);

            min_y = std::cmp::min(min_y, rect[1]);
            min_y = std::cmp::min(min_y, rect[3]);
            max_y = std::cmp::max(max_y, rect[1]);
            max_y = std::cmp::max(max_y, rect[3]);
        }

        println!("points={:?}", points);

        let mut t_patterns : Vec<bool> = vec![false;1<<4];
        t_patterns[BOTTOM_LEFT | BOTTOM_RIGHT] = true;
        t_patterns[TOP_LEFT | TOP_RIGHT] = true;
        t_patterns[TOP_RIGHT | BOTTOM_RIGHT] = true;
        t_patterns[TOP_LEFT | BOTTOM_LEFT] = true;

        let mut x_patterns : Vec<bool> = vec![false;1<<4];
        x_patterns[BOTTOM_LEFT | BOTTOM_RIGHT | TOP_LEFT | TOP_RIGHT] = true;

        let mut needed_corners : HashSet<usize> = [BOTTOM_LEFT, BOTTOM_RIGHT, TOP_LEFT, TOP_RIGHT].iter().cloned().collect();
        for (&ptr, &pattern) in points.iter() {
            if min_x < ptr.0 && ptr.0 < max_x || min_y < ptr.1 && ptr.1 < max_y {
                // interior point, either be t-formed or x-formed. 
                if !t_patterns[pattern] && !x_patterns[pattern] {
                    // println!("illegal interior point {:?} and pattern {}", ptr, pattern);
                    return false;
                }
            } else {
                // corner point, check there exists at least 4 corners, each with different types. 
                if !needed_corners.remove(&pattern) {
                    // println!("illegal corner point {:?} and pattern {}", ptr, pattern);
                    return false;
                }
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_391() {
        assert!(Solution::is_rectangle_cover(vec![vec![1,1,3,3],vec![3,1,4,2],vec![3,2,4,4],vec![1,3,2,4],vec![2,3,3,4]]));

        assert!(!Solution::is_rectangle_cover(vec![vec![1,1,2,3],vec![1,3,2,4],vec![3,1,4,2],vec![3,2,4,4]]));

        assert!(!Solution::is_rectangle_cover(vec![vec![1,1,3,3],vec![3,1,4,2],vec![1,3,2,4],vec![3,2,4,4]]));

        assert!(!Solution::is_rectangle_cover(vec![vec![1,1,3,3],vec![3,1,4,2],vec![1,3,2,4],vec![2,2,4,4]]));
    }
}
