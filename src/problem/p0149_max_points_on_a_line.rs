/**
 * [149] Max Points on a Line
 *
 * Given an array of points where points[i] = [xi, yi] represents a point on the X-Y plane, return the maximum number of points that lie on the same straight line.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/25/plane1.jpg" style="width: 300px; height: 294px;" />
 * Input: points = [[1,1],[2,2],[3,3]]
 * Output: 3
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/25/plane2.jpg" style="width: 300px; height: 294px;" />
 * Input: points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
 * Output: 4
 * 
 *  
 * Constraints:
 * 
 * 	1 <= points.length <= 300
 * 	points[i].length == 2
 * 	-10^4 <= xi, yi <= 10^4
 * 	All the points are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-points-on-a-line/
// discuss: https://leetcode.com/problems/max-points-on-a-line/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn gcd(a:i32, b:i32) -> i32 {if b==0 {a} else {Self::gcd(b, a%b)}}

    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut max : usize = 1;
        for i in 0..n-1 {
            let mut counter_per_slope : HashMap<(i32, i32), usize> = HashMap::new();
            let mut i_overlap_count : usize = 0;
            for j in (i+1)..n {
                let dx : i32 = points[j][0] - points[i][0];
                let dy : i32 = points[j][1] - points[i][1];
                if dx == 0 && dy == 0 {i_overlap_count+=1;continue}
                let d_gcd : i32 = Self::gcd(dx, dy);
                let dx : i32 = dx / d_gcd;
                let dy : i32 = dy / d_gcd;
                *counter_per_slope.entry((dx,dy)).or_insert(0) += 1;
                // println!("i={},j={},dx={},dy={}, count={}", i, j, dx, dy, counter_per_slope[&(dx,dy)]);
            }
            let mut i_max : usize = 0;
            if let Some(&m) = counter_per_slope.values().max() {
                i_max = m;
            }         
            max = std::cmp::max(max, i_max + i_overlap_count + 1);
        }
        max as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_149() {
        assert_eq!(
            Solution::max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            3
        );
        assert_eq!(
            Solution::max_points(vec![
                vec![1, 1],
                vec![3, 2],
                vec![5, 3],
                vec![4, 1],
                vec![2, 3],
                vec![1, 4]
            ]),
            4
        );
        assert_eq!(
            Solution::max_points(vec![vec![0, 0], vec![1, 65536], vec![65536, 0]]),
            2
        );
        assert_eq!(
            Solution::max_points(vec![vec![1, 1], vec![1, 1], vec![1, 1]]),
            3
        );
        assert_eq!(
            Solution::max_points(vec![
                vec![0, 9],
                vec![138, 429],
                vec![115, 359],
                vec![115, 359],
                vec![-30, -102],
                vec![230, 709],
                vec![-150, -686],
                vec![-135, -613],
                vec![-60, -248],
                vec![-161, -481],
                vec![207, 639],
                vec![23, 79],
                vec![-230, -691],
                vec![-115, -341],
                vec![92, 289],
                vec![60, 336],
                vec![-105, -467],
                vec![135, 701],
                vec![-90, -394],
                vec![-184, -551],
                vec![150, 774]
            ]),
            12
        )
    }
}
