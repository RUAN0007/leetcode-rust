/**
 * [518] Coin Change 2
 *
 * You are given coins of different denominations and a total amount of money. Write a function to compute the number of combinations that make up that amount. You may assume that you have infinite number of each kind of coin.
 * 
 * 
 * 
 * 
 *  
 * 
 * Example 1:
 * 
 * 
 * Input: amount = 5, coins = [1, 2, 5]
 * Output: 4
 * Explanation: there are four ways to make up the amount:
 * 5=5
 * 5=2+2+1
 * 5=2+1+1+1
 * 5=1+1+1+1+1
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: amount = 3, coins = [2]
 * Output: 0
 * Explanation: the amount of 3 cannot be made up just with coins of 2.
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: amount = 10, coins = [10] 
 * Output: 1
 * 
 * 
 *  
 * 
 * Note:
 * 
 * You can assume that
 * 
 * 
 * 	0 <= amount <= 5000
 * 	1 <= coin <= 5000
 * 	the number of coins is less than 500
 * 	the answer is guaranteed to fit into signed 32-bit integer
 * 
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/coin-change-2/
// discuss: https://leetcode.com/problems/coin-change-2/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut amount = amount as usize;
        let mut result = vec![0;amount+1];
        result[0] = 1; 
        for i in 1..=coins.len() {
            let this_coin = coins[i-1] as usize;
            for j in this_coin..=amount {
                result[j] += result[j-this_coin];
            }
        }
        result[amount]
    }

    pub fn change_2d(amount: i32, coins: Vec<i32>) -> i32 {
        let mut amount = amount as usize;
        // result[i][j] denote for # of ways to make up j amount with the first i coins. E.g, result[1][1] = # of ways to make up 1 amount with the first coin, coin[0]. 
        let mut result = vec![vec![0;amount+1];coins.len()+1];
        result[0][0] = 1;
        for i in 1..=coins.len() {
            result[i][0] = 1;
            for j in 0..=amount {
                let this_coin = coins[i-1] as usize;
                if this_coin <= j {
                    result[i][j] = result[i-1][j] // # of ways to make up j WITHOUT i-th coin. 
                                   + result[i][j-this_coin]; // # of ways to make up j WITH AT LEAST i-th coin. 
                } else {
                    result[i][j] = result[i-1][j];
                }
            }
        }

        result[coins.len()][amount]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_518() {
        assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
    }
}
