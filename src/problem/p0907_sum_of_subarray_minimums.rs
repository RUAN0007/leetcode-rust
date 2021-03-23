/**
 * [907] Sum of Subarray Minimums
 *
 * Given an array of integers arr, find the sum of min(b), where b ranges over every (contiguous) subarray of arr. Since the answer may be large, return the answer modulo 10^9 + 7.
 *  
 * Example 1:
 * 
 * Input: arr = [3,1,2,4]
 * Output: 17
 * Explanation: 
 * Subarrays are [3], [1], [2], [4], [3,1], [1,2], [2,4], [3,1,2], [1,2,4], [3,1,2,4]. 
 * Minimums are 3, 1, 2, 4, 1, 1, 2, 1, 1, 1.
 * Sum is 17.
 * 
 * Example 2:
 * 
 * Input: arr = [11,81,94,43,3]
 * Output: 444
 * 
 *  
 * Constraints:
 * 
 * 	1 <= arr.length <= 3 * 10^4
 * 	1 <= arr[i] <= 3 * 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-subarray-minimums/
// discuss: https://leetcode.com/problems/sum-of-subarray-minimums/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
#[derive(Debug)]
struct Pair(i32,  // num 
            i32,  // count
            i32   // sum of num*count for each pair in the stack, including the current one. 
        );
impl Solution {

    pub fn sum_subarray_mins(mut arr: Vec<i32>) -> i32 {
        let mut stack : Vec<Pair> = vec![]; 
        arr.reverse();
        let mut sum = 0i32;
        // stack with decreasing num
        for num in arr {
            let mut count = 1i32;
            while let Some(last) = stack.last() {
                let Pair(last_num, last_count, _) = *last;
                if num <= last_num {
                    stack.pop();
                    count +=last_count;
                } else {
                    break;
                }
            }
            let mut last_sum = 0i32;
            if let Some(last) = stack.last() {
                let Pair(_, _, this_last_sum) = *last;
                last_sum = this_last_sum;
            }

            let this_sum = (last_sum + num * count)  % (1000000000 + 7);
            sum = (sum + this_sum) % (1000000000 + 7);
            stack.push(Pair(num, count, this_sum));
            // println!("Stack: {:?}", stack);
        }
        sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_907() {
        assert_eq!(
            Solution::sum_subarray_mins(vec![3, 1, 2, 4]),
           17 
        );
    }
}
