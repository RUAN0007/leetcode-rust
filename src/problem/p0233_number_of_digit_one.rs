/**
 * [233] Number of Digit One
 *
 * Given an integer n, count the total number of digit 1 appearing in all non-negative integers less than or equal to n.
 *  
 * Example 1:
 * 
 * Input: n = 13
 * Output: 6
 * 
 * Example 2:
 * 
 * Input: n = 0
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	0 <= n <= 2 * 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-digit-one/
// discuss: https://leetcode.com/problems/number-of-digit-one/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut k : i32 = 1;
        let mut count : i32 = 0;

        let mut digit_count : usize = 0;
        let mut tmp : usize = n as usize;
        while tmp != 0 {
            digit_count+=1;
            tmp =  tmp / 10;
        }

        for i in 0..digit_count {
            let k : i32 = usize::pow(10usize, i as u32) as i32;
            // e.g, i=2, 12145 -> up: 121, down=45
            let up : i32 = n / k; // all prefix digits until i inclusive
            let down : i32 = n % k; // all suffix digits from i exclusive
            
            // count the 1-occurrence at bit-1, which happens once for every 10. 
            // up/10*k = 121 / 10 * 100 = 1200
            // [00100-01100], [01100-02100] ... [11100-12100]
            count += up / 10 * k;

            if up % 10 == 1 {
                // count i-th bit 1 in a subset range. 
                //   e.g, i = 0, in the range (12100, 12145)
                //   down + 1 = 46
                count += down + 1;
            } else if up % 10 > 1 {
                // 1 has surpassed on at i-th bit. Count the this range 
                //   e.g., i=3, up %2==2, additionally count i-th bit
                count += k;
            }
        }
        count
    }

    pub fn my_count_digit_one(n: i32) -> i32 {
        if  n <= 0 {
            return 0;
        }

        let mut digit_count : usize = 0;
        let mut highest_digit : usize = n as usize;
        let mut tmp : usize = n as usize;
        while tmp != 0 {
            digit_count+=1;
            highest_digit = tmp;
            tmp =  tmp / 10;
        }

        let highest_digit : usize = highest_digit as usize;
        let highest_order : usize = digit_count - 1;
        let mut result : usize = 0;
        for i in 0..highest_digit {
            if i == 1 {
                result += usize::pow(10usize, highest_order as u32);
            }
            if 0 < highest_order {
                result += highest_order * usize::pow(10usize, (highest_order - 1) as u32);
            }
        }

        let remain : usize = n as usize - highest_digit * usize::pow(10usize, highest_order as u32);
        if highest_digit == 1 {
            result += remain + 1;
        }
        println!("highest_order: {}, highest_digit: {}, remain: {}, this_result: {}", highest_order, highest_digit, remain, result);

        let rest_result : usize = Self::count_digit_one(remain as i32) as usize;
        result += rest_result;
        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_233() {
        assert_eq!(Solution::count_digit_one(13), 6);
    }
}
