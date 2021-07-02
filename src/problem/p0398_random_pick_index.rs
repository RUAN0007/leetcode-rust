/**
 * [398] Random Pick Index
 *
 * Given an integer array nums with possible duplicates, randomly output the index of a given target number. You can assume that the given target number must exist in the array.
 * Implement the Solution class:
 * 
 * 	Solution(int[] nums) Initializes the object with the array nums.
 * 	int pick(int target) Picks a random index i from nums where nums[i] == target. If there are multiple valid i's, then each index should have an equal probability of returning.
 * 
 *  
 * Example 1:
 * 
 * Input
 * ["Solution", "pick", "pick", "pick"]
 * [[[1, 2, 3, 3, 3]], [3], [1], [3]]
 * Output
 * [null, 4, 0, 2]
 * Explanation
 * Solution solution = new Solution([1, 2, 3, 3, 3]);
 * solution.pick(3); // It should return either index 2, 3, or 4 randomly. Each index should have equal probability of returning.
 * solution.pick(1); // It should return 0. Since in the array only nums[0] is equal to 1.
 * solution.pick(3); // It should return either index 2, 3, or 4 randomly. Each index should have equal probability of returning.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 2 * 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	target is an integer from nums.
 * 	At most 10^4 calls will be made to pick.
 * 
 */
// problem: https://leetcode.com/problems/random-pick-index/
// discuss: https://leetcode.com/problems/random-pick-index/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
struct Solution {
    num2positions : HashMap<i32, Vec<usize>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
       let mut num2positions : HashMap<i32, Vec<usize>> = HashMap::new();
       for (i, num) in nums.into_iter().enumerate() {
        num2positions.entry(num).or_insert(vec![]).push(i);
       }
       Self{num2positions : num2positions}
    }
    
    fn pick(&self, target: i32) -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let rand_pos : usize = rng.gen::<usize>() % self.num2positions[&target].len();
        self.num2positions[&target][rand_pos] as i32
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_398() {
    }
}
