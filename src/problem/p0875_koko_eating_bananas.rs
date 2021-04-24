/**
 * [875] Koko Eating Bananas
 *
 * Koko loves to eat bananas. There are n piles of bananas, the i^th pile has piles[i] bananas. The guards have gone and will come back in h hours.
 * Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses some pile of bananas and eats k bananas from that pile. If the pile has less than k bananas, she eats all of them instead and will not eat any more bananas during this hour.
 * Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.
 * Return the minimum integer k such that she can eat all the bananas within h hours.
 *  
 * Example 1:
 * 
 * Input: piles = [3,6,7,11], h = 8
 * Output: 4
 * 
 * Example 2:
 * 
 * Input: piles = [30,11,23,4,20], h = 5
 * Output: 30
 * 
 * Example 3:
 * 
 * Input: piles = [30,11,23,4,20], h = 6
 * Output: 23
 * 
 *  
 * Constraints:
 * 
 * 	1 <= piles.length <= 10^4
 * 	piles.length <= h <= 10^9
 * 	1 <= piles[i] <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/koko-eating-bananas/
// discuss: https://leetcode.com/problems/koko-eating-bananas/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut low_speed : i32 = 1;
        let mut high_speed : i32 = *piles.iter().max().unwrap();
        // < here as we are sure to find an answer. 
        while low_speed < high_speed {
            let mid_speed : i32 = (low_speed + high_speed) / 2;
            let hours : i32 = piles.iter().map(|&p|{
                if p % mid_speed == 0 {
                    p / mid_speed
                } else {
                    p / mid_speed + 1
                }
            }).sum();
            if hours <= h {
                // this speed is acceptable, and we try to continue searching lowimum
                high_speed = mid_speed;
            } else {
                low_speed = mid_speed + 1;
            }
        }
        low_speed
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_875() {
    }
}
