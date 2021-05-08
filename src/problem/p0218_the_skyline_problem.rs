/**
 * [218] The Skyline Problem
 *
 * A city's skyline is the outer contour of the silhouette formed by all the buildings in that city when viewed from a distance. Given the locations and heights of all the buildings, return the skyline formed by these buildings collectively.
 * The geometric information of each building is given in the array buildings where buildings[i] = [lefti, righti, heighti]:
 * 
 * 	lefti is the x coordinate of the left edge of the i^th building.
 * 	righti is the x coordinate of the right edge of the i^th building.
 * 	heighti is the height of the i^th building.
 * 
 * You may assume all buildings are perfect rectangles grounded on an absolutely flat surface at height 0.
 * The skyline should be represented as a list of "key points" sorted by their x-coordinate in the form [[x1,y1],[x2,y2],...]. Each key point is the left endpoint of some horizontal segment in the skyline except the last point in the list, which always has a y-coordinate 0 and is used to mark the skyline's termination where the rightmost building ends. Any ground between the leftmost and rightmost buildings should be part of the skyline's contour.
 * Note: There must be no consecutive horizontal lines of equal height in the output skyline. For instance, [...,[2 3],[4 5],[7 5],[11 5],[12 7],...] is not acceptable; the three lines of height 5 should be merged into one in the final output as such: [...,[2 3],[4 5],[12 7],...]
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/01/merged.jpg" style="width: 800px; height: 331px;" />
 * Input: buildings = [[2,9,10],[3,7,15],[5,12,12],[15,20,10],[19,24,8]]
 * Output: [[2,10],[3,15],[7,12],[12,0],[15,10],[20,8],[24,0]]
 * Explanation:
 * Figure A shows the buildings of the input.
 * Figure B shows the skyline formed by those buildings. The red points in figure B represent the key points in the output list.
 * 
 * Example 2:
 * 
 * Input: buildings = [[0,2,3],[2,5,3]]
 * Output: [[0,3],[5,0]]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= buildings.length <= 10^4
 * 	0 <= lefti < righti <= 2^31 - 1
 * 	1 <= heighti <= 2^31 - 1
 * 	buildings is sorted by lefti in non-decreasing order.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/the-skyline-problem/
// discuss: https://leetcode.com/problems/the-skyline-problem/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut critical_points : Vec<(i32, i32, usize, bool)> = vec![];
        for (i, building) in buildings.iter().enumerate() {
            critical_points.push((building[0], building[2], i, true));
            critical_points.push((building[1], building[2], i, false));
        }

        critical_points.sort_by(|x,y|{
            if x.0 < y.0 {
                return std::cmp::Ordering::Less;
            } else if x.0 > y.0 {
                return std::cmp::Ordering::Greater;
            
            // Equal horizontal coordinate
            } else if x.3 && !y.3{
                // place the starting rectangle ahead, which is x.
                return std::cmp::Ordering::Less;
            } else if y.3 && !x.3{
                // place the starting rectangle ahead, which is y.
                return std::cmp::Ordering::Greater;
            } else if x.3 && y.3 {
                // both the starting rectangle, sorted in reverse order by height. 
                return y.1.cmp(&x.1);
            } else {
                // both the ending rectangle, sorted in order by height. 
                return x.1.cmp(&y.1);
            }
        });
        // println!("sorted critical_points : {:?}", critical_points);

        let mut active_rects : HashMap<usize, i32> = HashMap::new();
        let mut cur_max_height : i32 = 0;
        let mut result : Vec<Vec<i32>> = vec![];
        for critical_point in critical_points.iter() {
            let x : i32 = critical_point.0;
            let y : i32 = critical_point.1;
            let rect_id : usize = critical_point.2;
            if critical_point.3 {
                // is left, a new rectangle starts
                active_rects.insert(rect_id, y);
                let new_max_height : i32 = *active_rects.values().max().unwrap();
                if cur_max_height < new_max_height {
                    result.push(vec![x, new_max_height]);
                    cur_max_height = new_max_height;
                }

            } else {
                // is left, a rectangle ends
                active_rects.remove(&rect_id);
                if let Some(&new_max_height) = active_rects.values().max() {
                    if new_max_height < cur_max_height {
                        result.push(vec![x, new_max_height]);
                        cur_max_height = new_max_height;
                    }
                } else {
                    // no active rectangle
                    result.push(vec![x, 0]);
                    cur_max_height = 0;
                }

            }

        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_218() {
        assert_eq!(Solution::get_skyline(vec![vec![2,9,10],vec![3,7,15],vec![5,12,12],vec![15,20,10],vec![19,24,8]]), vec![vec![2,10],vec![3,15],vec![7,12],vec![12,0],vec![15,10],vec![20,8],vec![24,0]]);

        // two continuous buildings
        assert_eq!(Solution::get_skyline(vec![vec![0,2,3],vec![2,5,3]]), vec![vec![0,3],vec![5,0]]);

        // three buildings stacked vertically
        assert_eq!(Solution::get_skyline(vec![vec![1,2,1],vec![1,2,2], vec![1,2,3]]), vec![vec![1,3],vec![2,0]]);
    }
}
