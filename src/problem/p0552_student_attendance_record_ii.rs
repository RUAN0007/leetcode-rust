/**
 * [552] Student Attendance Record II
 *
 * An attendance record for a student can be represented as a string where each character signifies whether the student was absent, late, or present on that day. The record only contains the following three characters:
 * 
 * 	'A': Absent.
 * 	'L': Late.
 * 	'P': Present.
 * 
 * Any student is eligible for an attendance award if they meet both of the following criteria:
 * 
 * 	The student was absent ('A') for strictly fewer than 2 days total.
 * 	The student was never late ('L') for 3 or more consecutive days.
 * 
 * Given an integer n, return the number of possible attendance records of length n that make a student eligible for an attendance award. The answer may be very large, so return it modulo 10^9 + 7.
 *  
 * Example 1:
 * 
 * Input: n = 2
 * Output: 8
 * Explanation: There are 8 records with length 2 that are eligible for an award:
 * "PP", "AP", "PA", "LP", "PL", "AL", "LA", "LL"
 * Only "AA" is not eligible because there are 2 absences (there need to be fewer than 2).
 * 
 * Example 2:
 * 
 * Input: n = 1
 * Output: 3
 * 
 * Example 3:
 * 
 * Input: n = 10101
 * Output: 183236316
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/student-attendance-record-ii/
// discuss: https://leetcode.com/problems/student-attendance-record-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
const MODULAR_I64 : i64 = 1000000000+7;
const MODULAR : i32 = 1000000000+7;
impl Solution {
    fn acc(n : usize, prev_absence_count : usize, prev_consecutive_late_count : usize, cache : &mut HashMap<(usize, usize, usize), i64>) -> i64 {
        if let Some(&cached) = cache.get(&(n, prev_absence_count, prev_consecutive_late_count)) {
            return cached;
        }

        if prev_absence_count == 2 || prev_consecutive_late_count == 3 {
            0i64
        } else if n == 0 {
            1i64
        } else {
            let this_absence_count : i64 = Self::acc(n-1, prev_absence_count+1, 0, cache);
            let this_late_count : i64 = Self::acc(n-1, prev_absence_count, prev_consecutive_late_count+1, cache);
            let this_presence_count : i64 = Self::acc(n-1, prev_absence_count, 0, cache);

            let sum : i64 = (this_absence_count + this_late_count + this_presence_count) % MODULAR_I64;
            cache.insert((n, prev_absence_count, prev_consecutive_late_count), sum);
            sum
        }
    }

    // stack overflow
    pub fn check_record_topdown(n: i32) -> i32 {
        let n : usize = n as usize;
        Self::acc(n, 0, 0, &mut HashMap::new()) as i32
    }

    pub fn check_record(n: i32) -> i32 {
        // only one a
        let mut a_x_ll : i64 = 0;
        let mut a_x_l : i64 = 0;

        // let mut a_x_p : i64 = 0;
        // let mut x_a : i64 = 1;
        let mut a_x : i64 = 1; // aggregate a_x_p + x_a

        // no a
        let mut x_ll : i64 = 0; // 2 trailing l
        let mut x_l : i64 = 1; // 1 trailing l
        let mut x : i64 = 1; // no trailing l

        let n : usize = n as usize;
        for i in 1..n {
            let past_a_x_ll = a_x_ll;
            let past_a_x_l = a_x_l;
            // let past_a_x_p = a_x_p;
            // let past_x_a = x_a;
            let past_a_x = a_x;
            let past_x_ll = x_ll;
            let past_x_l = x_l;
            let past_x = x;

            a_x_ll = (
                past_a_x_l // append l
            ) % MODULAR_I64;

            a_x_l = (
                // past_a_x_p // append l
                // + past_x_a // append l
                past_a_x // append l
            ) % MODULAR_I64;

            a_x =  (
                past_a_x_ll // append p
                + past_a_x_l  // append p
                + past_a_x  // append p
                + past_x_ll // append a
                + past_x_l  // append a
                + past_x  // append a
                 )
                % MODULAR_I64; // append p

            x_ll = (
                past_x_l // append l
            ) % MODULAR_I64;

            x_l = (
                past_x // append l
            ) % MODULAR_I64;

            x = (
                past_x_ll // append p
                + past_x_l // append p
                + past_x // append p
            ) % MODULAR_I64;
        }
        // ((a_x_ll + a_x_l + a_x_p + x_ll + x_l + x + x_a) % MODULAR_I64) as i32
        ((a_x_ll + a_x_l + x_ll + x_l + x + a_x) % MODULAR_I64) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_552() {
        assert_eq!(Solution::check_record(2), 8);
        assert_eq!(Solution::check_record(1), 3);
        assert_eq!(Solution::check_record(10101), 183236316);
    }
}
