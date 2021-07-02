/**
 * [313] Super Ugly Number
 *
 * A super ugly number is a positive integer whose prime factors are in the array primes.
 * Given an integer n and an array of integers primes, return the n^th super ugly number.
 * The n^th super ugly number is guaranteed to fit in a 32-bit signed integer.
 *  
 * Example 1:
 * 
 * Input: n = 12, primes = [2,7,13,19]
 * Output: 32
 * Explanation: [1,2,4,7,8,13,14,16,19,26,28,32] is the sequence of the first 12 super ugly numbers given primes = [2,7,13,19].
 * 
 * Example 2:
 * 
 * Input: n = 1, primes = [2,3,5]
 * Output: 1
 * Explanation: 1 has no prime factors, therefore all of its prime factors are in the array primes = [2,3,5].
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 10^6
 * 	1 <= primes.length <= 100
 * 	2 <= primes[i] <= 1000
 * 	primes[i] is guaranteed to be a prime number.
 * 	All the values of primes are unique and sorted in ascending order.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/super-ugly-number/
// discuss: https://leetcode.com/problems/super-ugly-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let n : usize = n as usize;
        let mut result : Vec<i32> = vec![!(1<<31);n];
        result[0] = 1;

        let plen : usize = primes.len();
        let mut prime_ptrs : Vec<usize> = vec![0;plen];
        for i in 1..n {
            for j in 0..plen {
                result[i] = std::cmp::min(result[i], result[prime_ptrs[j]] * primes[j]);
            }
            for j in 0..plen {
                if result[i] == result[prime_ptrs[j]] * primes[j] {
                    prime_ptrs[j]+=1;
                }
            }
        }
        result[n-1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_313() {
        assert_eq!(Solution::nth_super_ugly_number(12, vec![2, 7, 13, 19]), 32);
    }
}
