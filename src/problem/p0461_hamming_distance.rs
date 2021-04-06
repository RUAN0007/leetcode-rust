/**
 * [461] Hamming Distance
 *
 * The <a href="https://en.wikipedia.org/wiki/Hamming_distance" target="_blank">Hamming distance</a> between two integers is the number of positions at which the corresponding bits are different.
 * 
 * Given two integers x and y, calculate the Hamming distance.
 * 
 * Note:<br />
 * 0 &le; x, y < 2^31.
 * 
 * 
 * Example:
 * 
 * Input: x = 1, y = 4
 * 
 * Output: 2
 * 
 * Explanation:
 * 1   (0 0 0 1)
 * 4   (0 1 0 0)
 *        &uarr;   &uarr;
 * 
 * The above arrows point to positions where the corresponding bits are different.
 * 
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/hamming-distance/
// discuss: https://leetcode.com/problems/hamming-distance/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn digit1_count(mut x : i32) -> usize {
        let mut count = 0;
        while x != 0 {
            x = x & (x-1);
            count+=1;
        }
        count
    }
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        Self::digit1_count(x^y) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_461() {
    }
}
