/**
 * [264] Ugly Number II
 *
 * An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.
 * Given an integer n, return the n^th ugly number.
 *  
 * Example 1:
 * 
 * Input: n = 10
 * Output: 12
 * Explanation: [1, 2, 3, 4, 5, 6, 8, 9, 10, 12] is the sequence of the first 10 ugly numbers.
 * 
 * Example 2:
 * 
 * Input: n = 1
 * Output: 1
 * Explanation: 1 has no prime factors, therefore all of its prime factors are limited to 2, 3, and 5.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 1690
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/ugly-number-ii/
// discuss: https://leetcode.com/problems/ugly-number-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        // assume ugly[i] is the i-th ugly num, and ugly[0] = 1;
        // Seperate ugly number into three lists:
        //   S2: ugly[0]*2, ugly[1]*2, ugly[2]*2, ..., ugly[l]*2
        //   S3: ugly[0]*3, ugly[1]*3, ugly[2]*3, ..., ugly[m]*3
        //   S5: ugly[0]*5, ugly[1]*5, ugly[2]*5, ..., ugly[p]*5

        // Increment l,m and n, and merge the above three lists to get the ugly[n]
        let n : usize = n as usize;
        let mut ugly : Vec<i32> = vec![1;n];
        let mut l = 0usize;
        let mut m = 0usize;
        let mut p = 0usize;

        for i in 1..n {
            ugly[i] = *[ugly[l]*2, ugly[m]*3, ugly[p]*5].iter().min().unwrap();
            if ugly[i] == ugly[l] * 2 {l+=1;}
            if ugly[i] == ugly[m] * 3 {m+=1;}
            if ugly[i] == ugly[p] * 5 {p+=1;}
        }
        ugly[n-1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_264() {
        // assert_eq!(Solution::nth_ugly_number(1), 1);
        assert_eq!(Solution::nth_ugly_number(10), 12);
        assert_eq!(Solution::nth_ugly_number(3), 3);
    }
}
