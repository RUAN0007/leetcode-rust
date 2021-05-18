/**
 * [352] Data Stream as Disjoint Intervals
 *
 * Given a data stream input of non-negative integers a1, a2, ..., an, summarize the numbers seen so far as a list of disjoint intervals.
 * Implement the SummaryRanges class:
 * 
 * 	SummaryRanges() Initializes the object with an empty stream.
 * 	void addNum(int val) Adds the integer val to the stream.
 * 	int[][] getIntervals() Returns a summary of the integers in the stream currently as a list of disjoint intervals [starti, endi].
 * 
 *  
 * Example 1:
 * 
 * Input
 * ["SummaryRanges", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals"]
 * [[], [1], [], [3], [], [7], [], [2], [], [6], []]
 * Output
 * [null, null, [[1, 1]], null, [[1, 1], [3, 3]], null, [[1, 1], [3, 3], [7, 7]], null, [[1, 3], [7, 7]], null, [[1, 3], [6, 7]]]
 * Explanation
 * SummaryRanges summaryRanges = new SummaryRanges();
 * summaryRanges.addNum(1);      // arr = [1]
 * summaryRanges.getIntervals(); // return [[1, 1]]
 * summaryRanges.addNum(3);      // arr = [1, 3]
 * summaryRanges.getIntervals(); // return [[1, 1], [3, 3]]
 * summaryRanges.addNum(7);      // arr = [1, 3, 7]
 * summaryRanges.getIntervals(); // return [[1, 1], [3, 3], [7, 7]]
 * summaryRanges.addNum(2);      // arr = [1, 2, 3, 7]
 * summaryRanges.getIntervals(); // return [[1, 3], [7, 7]]
 * summaryRanges.addNum(6);      // arr = [1, 2, 3, 6, 7]
 * summaryRanges.getIntervals(); // return [[1, 3], [6, 7]]
 * 
 *  
 * Constraints:
 * 
 * 	0 <= val <= 10^4
 * 	At most 3 * 10^4 calls will be made to addNum and getIntervals.
 * 
 *  
 * Follow up: What if there are lots of merges and the number of disjoint intervals is small compared to the size of the data stream?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/data-stream-as-disjoint-intervals/
// discuss: https://leetcode.com/problems/data-stream-as-disjoint-intervals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct SummaryRanges {
    ranges : BTreeMap<i32, i32>,
}

use std::collections::BTreeMap;
use std::ops::RangeToInclusive;
use std::ops::RangeFrom;
use std::ops::RangeFull;

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {

    /** Initialize your data structure here. */
    fn new() -> Self {
       SummaryRanges{ranges : BTreeMap::new()} 
    }
    
    fn add_num(&mut self, val: i32) {
        let mut this_start : i32 = val;
        if let Some((&left_start, &left_end)) = self.ranges.range(RangeToInclusive{end : val}).next_back() {
            if left_end < val - 1 {
                // do nothing
            } else if left_end == val - 1 {
                self.ranges.remove(&left_start).expect("left_start must exists");
                this_start = left_start;
            } else {
                // this num is already in range. 
                return
            }
        };

        let mut this_end : i32 = val;
        if let Some((&right_start, &right_end)) = self.ranges.range(RangeFrom{start : val}).next() {
            if right_start == val + 1 {
                self.ranges.remove(&right_start).expect("right_start must exists");
                this_end = right_end;
            }
        };

        self.ranges.insert(this_start, this_end);
    }
    
    fn get_intervals(&self) -> Vec<Vec<i32>> {
        let mut result = vec![];
        for (&start, &end) in self.ranges.range(RangeFull) {
            result.push(vec![start, end]);
        }
        result
    }
}

/**
 * Your SummaryRanges object will be instantiated and called as such:
 * let obj = SummaryRanges::new();
 * obj.add_num(val);
 * let ret_2: Vec<Vec<i32>> = obj.get_intervals();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_352() {
    }
}
