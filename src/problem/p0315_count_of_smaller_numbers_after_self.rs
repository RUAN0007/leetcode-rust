/**
 * [315] Count of Smaller Numbers After Self
 *
 * You are given an integer array nums and you have to return a new counts array. The counts array has the property where counts[i] is the number of smaller elements to the right of nums[i].
 *  
 * Example 1:
 * 
 * Input: nums = [5,2,6,1]
 * Output: [2,1,1,0]
 * Explanation:
 * To the right of 5 there are 2 smaller elements (2 and 1).
 * To the right of 2 there is only 1 smaller element (1).
 * To the right of 6 there is 1 smaller element (1).
 * To the right of 1 there is 0 smaller element.
 * 
 * Example 2:
 * 
 * Input: nums = [-1]
 * Output: [0]
 * 
 * Example 3:
 * 
 * Input: nums = [-1,-1]
 * Output: [0,0]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	-10^4 <= nums[i] <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-of-smaller-numbers-after-self/
// discuss: https://leetcode.com/problems/count-of-smaller-numbers-after-self/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn merge_sort_index (nums : &Vec<i32>, right_smaller_count : &mut Vec<i32>, indices : Vec<usize>, level : usize) -> Vec<usize> {
        let pad : String = (0..level).map(|_|{"...."}).collect();
        let n : usize = indices.len();
        if n == 1 {return indices}
        let mid : usize = n / 2;
        let left : Vec<usize> = indices.iter().take(mid).cloned().collect();
        let right : Vec<usize> = indices.iter().skip(mid).cloned().collect();
        let mut left : Vec<usize> = Self::merge_sort_index(nums, right_smaller_count, left, level + 1);
        let mut right : Vec<usize> = Self::merge_sort_index(nums, right_smaller_count, right, level + 1);

        let mut sorted_inversed : Vec<usize> = vec![];
        // println!("{}left={:?}, right={:?}", pad, left, right);
        while left.len() !=0 || right.len() != 0 {
            let is_left_greater : bool = (right.len() == 0 || left.len() !=0 && nums[*left.last().unwrap()] > nums[*right.last().unwrap()]);

            if is_left_greater {
                let left_greatest_index : usize = left.pop().unwrap();
                sorted_inversed.push(left_greatest_index);
                // Shift this num from the left to right, count the surpassed right nums.
                right_smaller_count[left_greatest_index] += right.len() as i32;
            } else {
                let right_greatest_index : usize = right.pop().unwrap();
                sorted_inversed.push(right_greatest_index);
            }
        }
        let sorted_index : Vec<usize> = sorted_inversed.into_iter().rev().collect();

        // println!("{}sorted={:?}, right_smaller={:?}", pad, sorted_index, right_smaller_count);
        sorted_index
    }
    
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut right_smaller_count : Vec<i32> = vec![0i32; nums.len()];
        let indices : Vec<usize> = (0..(nums.len())).collect();
        Self::merge_sort_index(&nums, &mut right_smaller_count, indices, 0);
        right_smaller_count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_315() {
        assert_eq!(Solution::count_smaller(vec![5,2,6,1]), vec![2,1,1,0]);
    }
}
