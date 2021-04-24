/**
 * [322] Coin Change
 *
 * You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
 * Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.
 * You may assume that you have an infinite number of each kind of coin.
 *  
 * Example 1:
 * 
 * Input: coins = [1,2,5], amount = 11
 * Output: 3
 * Explanation: 11 = 5 + 5 + 1
 * 
 * Example 2:
 * 
 * Input: coins = [2], amount = 3
 * Output: -1
 * 
 * Example 3:
 * 
 * Input: coins = [1], amount = 0
 * Output: 0
 * 
 * Example 4:
 * 
 * Input: coins = [1], amount = 1
 * Output: 1
 * 
 * Example 5:
 * 
 * Input: coins = [1], amount = 2
 * Output: 2
 * 
 *  
 * Constraints:
 * 
 * 	1 <= coins.length <= 12
 * 	1 <= coins[i] <= 2^31 - 1
 * 	0 <= amount <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/coin-change/
// discuss: https://leetcode.com/problems/coin-change/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        let amount : usize = amount as usize;
        let mut result : Vec<Vec<i32>> = vec![vec![-1;amount+1];coins.len()+1];
        // result[i][j] represent the ways to make amount i with the first j coins. 

        result[0][0] = 0;
        for i in 1..=coins.len() {
            result[i][0] = 0;
            for j in 1..=amount {
                let this_coin : usize = coins[i-1] as usize;

                //make up only using the first i-1 coints.
                if result[i-1][j] != -1 {
                    result[i][j] = result[i-1][j];
                }

                if this_coin <= j && result[i][j-this_coin] != -1 {
                    if result[i][j] != -1 {
                        result[i][j] = std::cmp::min(result[i][j], 1 + result[i][j-this_coin]);
                    } else {
                        result[i][j] = 1 + result[i][j-this_coin];
                    }
                }
            }
        }
        // println!("result={:?}", result);
        result[coins.len()][amount]
    }

    // pub fn coin_change_old(mut coins: Vec<i32>, amount: i32) -> i32 {
    //     let k_large = 100000;
    //     coins.sort();
    //     let amount = amount as usize;
    //     let mut change_ways = vec![k_large; amount+1];
    //     change_ways[0] = 0;
    //     for i in 1..=amount {
    //         let i = i as usize;
    //         for &coin_num in &coins {
    //             let coin_num = coin_num as usize;
    //             // if i is not changeable from i-1, i-2 and i-5, 
    //             //   change_ways[i] will still be greater than k_large due to the min op. 
    //             if coin_num <= i {
    //                 change_ways[i] = std::cmp::min(change_ways[i], change_ways[i-coin_num]+1);
    //             }
    //             // println!("{:?}", change_ways);
    //         }
    //     }
    //     if change_ways[amount] >= k_large {
    //         -1
    //     } else {
    //         change_ways[amount]
    //     }
    // }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_322() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
        assert_eq!(Solution::coin_change(vec![2, 5, 10, 1], 27), 4);
    }
}
