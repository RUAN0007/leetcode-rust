/**
 * [137] Single Number II
 *
 * Given an integer array nums where every element appears three times except for one, which appears exactly once. Find the single element and return it.
 *  
 * Example 1:
 * Input: nums = [2,2,3,2]
 * Output: 3
 * Example 2:
 * Input: nums = [0,1,0,1,0,1,99]
 * Output: 99
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 3 * 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	Each element in nums appears exactly three times except for one element which appears once.
 * 
 *  
 * Follow up: Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/single-number-ii/
// discuss: https://leetcode.com/problems/single-number-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // for each bit 0<=i<31, x1[i]x0[i] forms a cyclic counter with a period of 3. xj[i] stands for the i-th bit in xj. Each i corresponds to a counter. 

        let mut x0 = 0;
        let mut x1 = 0;

        for &num in &nums {
            x1 ^= (x0 & num);
            x0 ^= num;

            // for each i, mask[i]=0 if and only if x1[i]=x0[i], which implies the bit counter has reached to 3. 
            let mask = !(x1 & x0);

            // for each bit i/counter, reset to 0 if it reaches 0b11 (3). 
            x1&=mask;
            x0&=mask;
        }
        // p=1 can be structured as 0b01. The 1-bit corresponds to x0, which can be proved equal to the single element e, as below: 
        // If e[i]=0, the i-th counter is always zero and hence xj[i]=e[i]=0 for any j. 
        // If e[i]=1, the i-th counter is incremented for p times and hence x1[i]x0[i]=p. Given p[j]=1, xj[i]=e[i]=1. 
        // Hence, as long as we select a j such that p[j]=1, xj[i]=e[i] for all i and xj=e
        x0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_137() {
        assert_eq!(Solution::single_number(vec![0, 0, 0, 1, 1, 1, 5]), 5);
    }
}
