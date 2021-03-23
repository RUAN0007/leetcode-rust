/**
 * [56] Merge Intervals
 *
 * Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.
 *  
 * Example 1:
 * 
 * Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
 * Output: [[1,6],[8,10],[15,18]]
 * Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].
 * 
 * Example 2:
 * 
 * Input: intervals = [[1,4],[4,5]]
 * Output: [[1,5]]
 * Explanation: Intervals [1,4] and [4,5] are considered overlapping.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= intervals.length <= 10^4
 * 	intervals[i].length == 2
 * 	0 <= starti <= endi <= 10^4
 * 
 */


pub struct Solution {}

// problem: https://leetcode.com/problems/merge-intervals/
// discuss: https://leetcode.com/problems/merge-intervals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by(|a,b| a[0].partial_cmp(&b[0]).unwrap());

        let mut result = vec![];
        let len = intervals.len();
        if 0 < len {
            let mut cur_interval = intervals[0].clone();
            for i in 1..len {
                let cur_end = cur_interval[1];
                let next_start = intervals[i][0];
                let next_end = intervals[i][1];
                if (next_end < cur_end) {
                    continue;
                } else if cur_end < next_start {
                    result.push(cur_interval);
                    cur_interval = intervals[i].clone();
                } else {
                    cur_interval[1] = next_end;
                }
            }
            result.push(cur_interval);
        }         
        result
    }
}

// submission codes end