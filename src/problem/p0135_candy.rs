/**
 * [135] Candy
 *
 * There are n children standing in a line. Each child is assigned a rating value given in the integer array ratings.
 * You are giving candies to these children subjected to the following requirements:
 * 
 * 	Each child must have at least one candy.
 * 	Children with a higher rating get more candies than their neighbors.
 * 
 * Return the minimum number of candies you need to have to distribute the candies to the children.
 *  
 * Example 1:
 * 
 * Input: ratings = [1,0,2]
 * Output: 5
 * Explanation: You can allocate to the first, second and third child with 2, 1, 2 candies respectively.
 * 
 * Example 2:
 * 
 * Input: ratings = [1,2,2]
 * Output: 4
 * Explanation: You can allocate to the first, second and third child with 1, 2, 1 candies respectively.
 * The third child gets 1 candy because it satisfies the above two conditions.
 * 
 *  
 * Constraints:
 * 
 * 	n == ratings.length
 * 	1 <= n <= 2 * 10^4
 * 	0 <= ratings[i] <= 2 * 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/candy/
// discuss: https://leetcode.com/problems/candy/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n : usize = ratings.len();
        let mut result : Vec<i32> = vec![1i32;n];
        for i in 1..n {
            if ratings[i-1] < ratings[i] {
                result[i] = result[i-1] + 1;
            }
        }

        for i in (0..(n-1)).rev() {
            if ratings[i] > ratings[i+1] {
                result[i] = std::cmp::max(result[i], result[i+1] + 1);
            }
        }
        result.iter().sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_135() {
        assert_eq!(Solution::candy(vec![3, 2, 1, 2, 3]), 11);
        assert_eq!(Solution::candy(vec![2, 2, 1, 2, 2]), 7);
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
        assert_eq!(Solution::candy(vec![1, 1, 1, 1, 1, 1]), 6);
        assert_eq!(Solution::candy(vec![1, 2, 2, 2, 2, 2, 2, 0]), 10);
    }
}
