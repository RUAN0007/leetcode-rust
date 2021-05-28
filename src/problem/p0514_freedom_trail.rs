/**
 * [514] Freedom Trail
 *
 * In the video game Fallout 4, the quest "Road to Freedom" requires players to reach a metal dial called the "Freedom Trail Ring" and use the dial to spell a specific keyword to open the door.
 * Given a string ring that represents the code engraved on the outer ring and another string key that represents the keyword that needs to be spelled, return the minimum number of steps to spell all the characters in the keyword.
 * Initially, the first character of the ring is aligned at the "12:00" direction. You should spell all the characters in key one by one by rotating ring clockwise or anticlockwise to make each character of the string key aligned at the "12:00" direction and then by pressing the center button.
 * At the stage of rotating the ring to spell the key character key[i]:
 * <ol>
 * 	You can rotate the ring clockwise or anticlockwise by one place, which counts as one step. The final purpose of the rotation is to align one of ring's characters at the "12:00" direction, where this character must equal key[i].
 * 	If the character key[i] has been aligned at the "12:00" direction, press the center button to spell, which also counts as one step. After the pressing, you could begin to spell the next character in the key (next stage). Otherwise, you have finished all the spelling.
 * </ol>
 *  
 * Example 1:
 * <img src="https://assets.leetcode.com/uploads/2018/10/22/ring.jpg" style="width: 450px; height: 450px;" />
 * Input: ring = "godding", key = "gd"
 * Output: 4
 * Explanation:
 * For the first key character 'g', since it is already in place, we just need 1 step to spell this character. 
 * For the second key character 'd', we need to rotate the ring "godding" anticlockwise by two steps to make it become "ddinggo".
 * Also, we need 1 more step for spelling.
 * So the final output is 4.
 * 
 * Example 2:
 * 
 * Input: ring = "godding", key = "godding"
 * Output: 13
 * 
 *  
 * Constraints:
 * 
 * 	1 <= ring.length, key.length <= 100
 * 	ring and key consist of only lower case English letters.
 * 	It is guaranteed that key could always be spelled by rotating ring.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/freedom-trail/
// discuss: https://leetcode.com/problems/freedom-trail/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn solve(ring : &Vec<char>, key : &Vec<char>, ring_pos : usize, key_pos : usize, cache : &mut HashMap<(usize, usize), i32>) -> i32 {
        if key_pos == key.len() {
            return 0;
        }
        if let Some(&cached) = cache.get(&(ring_pos, key_pos)) {
            return cached
        }

        let ring_len : usize = ring.len();
        let mut min_total_step : i32 = 10000000;
        for i in 0..ring_len {
            if ring[i] == key[key_pos] {
                let clockwise_rotation : usize = (i + ring_len - ring_pos) % ring_len;
                let anticlockwise_rotation : usize = (ring_pos + ring_len - i) % ring_len;
                // println!("\ti={},c_rotation={}, anti_rotation={}", i, clockwise_rotation, anticlockwise_rotation);
                let this_step : i32 = 1 + std::cmp::min(clockwise_rotation, anticlockwise_rotation) as i32;

                let total_step : i32 = this_step + Self::solve(ring, key, i, key_pos+1, cache);
                min_total_step = std::cmp::min(min_total_step, total_step);
            }
        }
        // println!("ring_pos={}, key_pos={}, min_total_step={}", ring_pos, key_pos, min_total_step);
        cache.insert((ring_pos, key_pos), min_total_step);
        min_total_step
    }

    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut cache : HashMap<(usize, usize), i32> = HashMap::new();
        let ring : Vec<char> = ring.chars().collect();
        let key : Vec<char> = key.chars().collect();

        Self::solve(&ring, &key, 0usize, 0usize, &mut cache)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_514() {
        assert_eq!(Solution::find_rotate_steps("godding".to_owned(), "gd".to_owned()), 4);
        assert_eq!(Solution::find_rotate_steps("godding".to_owned(), "godding".to_owned()), 13);
    }
}
