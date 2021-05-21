/**
 * [403] Frog Jump
 *
 * A frog is crossing a river. The river is divided into some number of units, and at each unit, there may or may not exist a stone. The frog can jump on a stone, but it must not jump into the water.
 * Given a list of stones' positions (in units) in sorted ascending order, determine if the frog can cross the river by landing on the last stone. Initially, the frog is on the first stone and assumes the first jump must be 1 unit.
 * If the frog's last jump was k units, its next jump must be either k - 1, k, or k + 1 units. The frog can only jump in the forward direction.
 *  
 * Example 1:
 * 
 * Input: stones = [0,1,3,5,6,8,12,17]
 * Output: true
 * Explanation: The frog can jump to the last stone by jumping 1 unit to the 2nd stone, then 2 units to the 3rd stone, then 2 units to the 4th stone, then 3 units to the 6th stone, 4 units to the 7th stone, and 5 units to the 8th stone.
 * 
 * Example 2:
 * 
 * Input: stones = [0,1,2,3,4,8,9,11]
 * Output: false
 * Explanation: There is no way to jump to the last stone as the gap between the 5th and 6th stone is too large.
 * 
 *  
 * Constraints:
 * 
 * 	2 <= stones.length <= 2000
 * 	0 <= stones[i] <= 2^31 - 1
 * 	stones[0] == 0
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/frog-jump/
// discuss: https://leetcode.com/problems/frog-jump/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let mut allowable_steps_on_stone : HashMap<i32, HashSet<i32>> = HashMap::new();

        let last_stone : i32 = *stones.last().unwrap();
        for &stone in stones.iter() {
            allowable_steps_on_stone.insert(stone,HashSet::new());
        }
        allowable_steps_on_stone.get_mut(&0i32).unwrap().insert(1);

        for &stone in stones.iter() {
            let mut stone_added_steps : HashMap<i32, HashSet<i32>> = HashMap::new();
            for &allowable_step in allowable_steps_on_stone[&stone].iter() {
            // for i in 0..allowable_steps_on_stone[&stone].len() {

               let reach : i32 = stone + allowable_step; 
               if reach == last_stone {return true;}

               stone_added_steps.entry(reach).or_insert(HashSet::new()).insert(allowable_step);
               stone_added_steps.entry(reach).or_insert(HashSet::new()).insert(allowable_step + 1);
               if allowable_step - 1 > 0 {
                 stone_added_steps.entry(reach).or_insert(HashSet::new()).insert(allowable_step - 1);
               }
            }

            for (&stone, added_steps) in stone_added_steps.iter() {
                if let Some(allowable_steps) = allowable_steps_on_stone.get_mut(&stone) {
                    for &added_step in added_steps.iter() {
                        allowable_steps.insert(added_step);
                    }
                }
            }

        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_403() {
        assert!(Solution::can_cross(vec![0,1,3,5,6,8,12,17]));
        assert!(!Solution::can_cross(vec![0,1,2,3,4,8,9,11]));
    }
}
