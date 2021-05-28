/**
 * [546] Remove Boxes
 *
 * You are given several boxes with different colors represented by different positive numbers.
 * You may experience several rounds to remove boxes until there is no box left. Each time you can choose some continuous boxes with the same color (i.e., composed of k boxes, k >= 1), remove them and get k * k points.
 * Return the maximum points you can get.
 *  
 * Example 1:
 * 
 * Input: boxes = [1,3,2,2,2,3,4,3,1]
 * Output: 23
 * Explanation:
 * [1, 3, 2, 2, 2, 3, 4, 3, 1] 
 * ----> [1, 3, 3, 4, 3, 1] (3*3=9 points) 
 * ----> [1, 3, 3, 3, 1] (1*1=1 points) 
 * ----> [1, 1] (3*3=9 points) 
 * ----> [] (2*2=4 points)
 * 
 * Example 2:
 * 
 * Input: boxes = [1,1,1]
 * Output: 9
 * 
 * Example 3:
 * 
 * Input: boxes = [1]
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= boxes.length <= 100
 * 	1 <= boxes[i] <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-boxes/
// discuss: https://leetcode.com/problems/remove-boxes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    // prev_count counts the previous consecutive occurrence of boxes[left]
    // left and right are inclusive
    pub fn compute(boxes : &Vec<i32>, left : usize, right : usize, prev_count : usize, cache : &mut HashMap<(usize, usize, usize), i32>) -> i32 {
        if left > right { return 0; }
        if let Some(&cached) = cache.get(&(left, right, prev_count)) {
            return cached;
        }
        let mut cur_max : i32 = prev_count as i32 * prev_count as i32 + Self::compute(boxes, left + 1, right, 1, cache);

        for i in left+1..=right {
            if boxes[left] == boxes[i] {
                // process [left+1,i-1] so that boxes[left], boxes[i] can be consecutive when processing [i,right].
                let this_max = Self::compute(boxes, left + 1, i - 1, 1, cache) + Self::compute(boxes, i, right, prev_count+1, cache);
                cur_max = std::cmp::max(cur_max, this_max);
            }
        }
        cache.insert((left, right, prev_count), cur_max);
        cur_max
    }

    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        Self::compute(&boxes, 0, boxes.len()-1, 1, &mut HashMap::new())
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_546() {
        assert_eq!(Solution::remove_boxes(vec![1,3,2,2,2,3,4,3,1]), 23);
        assert_eq!(Solution::remove_boxes(vec![1,1,1]), 9);
        assert_eq!(Solution::remove_boxes(vec![1]), 1);
    }
}
