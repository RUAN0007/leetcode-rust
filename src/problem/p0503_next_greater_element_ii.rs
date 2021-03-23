/**
 * [503] Next Greater Element II
 *
 * 
 * Given a circular array (the next element of the last element is the first element of the array), print the Next Greater Number for every element. The Next Greater Number of a number x is the first greater number to its traversing-order next in the array, which means you could search circularly to find its next greater number. If it doesn't exist, output -1 for this number.
 * 
 * 
 * Example 1:<br />
 * 
 * Input: [1,2,1]
 * Output: [2,-1,2]
 * Explanation: The first 1's next greater number is 2; </br>The number 2 can't find next greater number; </br>The second 1's next greater number needs to search circularly, which is also 2.
 * 
 * 
 * 
 * Note:
 * The length of given array won't exceed 10000.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/next-greater-element-ii/
// discuss: https://leetcode.com/problems/next-greater-element-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        // Locate the max_num and its index.
        let (mut max_id, mut max_num) : (usize, i32) = (0, -10000000);
        for (idx, &num) in nums.iter().enumerate() {
            if max_num < num {
                max_id = idx;
                max_num = num;
            }
        }

        // println!("max_id: {}, max_num: {}", max_id, max_num);
        // From the max num and circularly iterate to the left
        //   Update the result stack. 
        let size = nums.len();
        let mut result = vec![-1; size];
        let mut stack = vec![];
        for i in 0..size {
            let idx = (max_id+size-i) % size;
            let num = nums[idx];
            while let Some(&last) = stack.last() {
               if last <= num {
                   stack.pop();
               } else {
                   break;
               }
            }
            if let Some(&last) = stack.last() {
                result[idx] = last;
            }
            stack.push(num);
            println!("idx: {}, num: {}, stack: {:?}", idx, num, stack);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_503() {
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2, 1]),
            vec![2, -1, 2]
        );
    }
}
