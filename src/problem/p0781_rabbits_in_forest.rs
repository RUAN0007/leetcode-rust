/**
 * [781] Rabbits in Forest
 *
 * In a forest, each rabbit has some color. Some subset of rabbits (possibly all of them) tell you how many other rabbits have the same color as them. Those answers are placed in an array.
 * 
 * Return the minimum number of rabbits that could be in the forest.
 * 
 * 
 * Examples:
 * Input: answers = [1, 1, 2]
 * Output: 5
 * Explanation:
 * The two rabbits that answered "1" could both be the same color, say red.
 * The rabbit than answered "2" can't be red or the answers would be inconsistent.
 * Say the rabbit that answered "2" was blue.
 * Then there should be 2 other blue rabbits in the forest that didn't answer into the array.
 * The smallest possible number of rabbits in the forest is therefore 5: 3 that answered plus 2 that didn't.
 * 
 * Input: answers = [10, 10, 10]
 * Output: 11
 * 
 * Input: answers = []
 * Output: 0
 * 
 * 
 * Note:
 * 
 * <ol>
 * 	answers will have length at most 1000.
 * 	Each answers[i] will be an integer in the range [0, 999].
 * </ol>
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rabbits-in-forest/
// discuss: https://leetcode.com/problems/rabbits-in-forest/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        for answer in answers { 
            if let Some(c) = count.get_mut(&answer) {
                *c += 1;
            } else {
                count.insert(answer, 1);
            }
        }

        let mut result = 0;

        for (num, count) in count {
            if count % (num+1) == 0 {
                result += count;
            } else {
                result += (count / (num+1) + 1) * (num+1);
            }
            println!("num={}, count={} result={}", num, count, result);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_781() {
        assert_eq!(Solution::num_rabbits(vec![1, 1, 2]), 5);
        assert_eq!(Solution::num_rabbits(vec![10, 10, 10]), 11);
        assert_eq!(Solution::num_rabbits(vec![]), 0);
    }
}
