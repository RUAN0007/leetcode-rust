/**
 * [188] Best Time to Buy and Sell Stock IV
 *
 * You are given an integer array prices where prices[i] is the price of a given stock on the i^th day, and an integer k.
 * Find the maximum profit you can achieve. You may complete at most k transactions.
 * Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
 *  
 * Example 1:
 * 
 * Input: k = 2, prices = [2,4,1]
 * Output: 2
 * Explanation: Buy on day 1 (price = 2) and sell on day 2 (price = 4), profit = 4-2 = 2.
 * 
 * Example 2:
 * 
 * Input: k = 2, prices = [3,2,6,5,0,3]
 * Output: 7
 * Explanation: Buy on day 2 (price = 2) and sell on day 3 (price = 6), profit = 6-2 = 4. Then buy on day 5 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
 * 
 *  
 * Constraints:
 * 
 * 	0 <= k <= 100
 * 	0 <= prices.length <= 1000
 * 	0 <= prices[i] <= 1000
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        // Since a txn must span at least two days, one day to buy and one day to sell, when k >n/2, it is equivalent to k = inf. 
        // We skip this optimization for clarity. 

        let k = k as usize;
        let mut last_no_stock_balances : Vec<i32> = vec![0;k+1];
        let min_i32 = -2_147_483_648i32;
        let mut last_with_stock_balances : Vec<i32> = vec![min_i32;k+1];
        for price in prices.iter() {
            // iterate reversely so that last_no_stock_balances[k-1] = no_stock_balances[i-1][k-1]. If not reverse, it would be last_no_stock_balances[k-1] = no_stock_balances[i][k-1]
            for kk in (1..=k).rev() {
                last_no_stock_balances[kk] = std::cmp::max(last_no_stock_balances[kk], last_with_stock_balances[kk] + price);

                last_with_stock_balances[kk] = std::cmp::max(last_with_stock_balances[kk], last_no_stock_balances[kk-1] - price);
            }
        }
        last_no_stock_balances[k]
    }

    pub fn max_profit1(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let n = prices.len();
        let mut balances : Vec<Vec<i32>> = vec![vec![0;n];k+1];
        for i_k in 1..=k {
            let mut min : i32 = prices[0];
            for j_n in 1..n {
                min = std::cmp::min(min, prices[j_n]-balances[i_k-1][j_n-1]);
                balances[i_k][j_n] = std::cmp::max(balances[i_k][j_n-1], prices[j_n]-min);
            }
        }
        balances[k][n-1]
    }

    pub fn my_max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let i32_min = -2_147_483_648i32;
        // on_hold[i] represents the asset after the i-th buying
        let mut on_hold : Vec<i32> = vec![i32_min;k+1];
        // no_hold[i] represents the asset after the i-th selling
        let mut no_hold : Vec<i32> = vec![i32_min;k+1];
        no_hold[0] = 0;

        for (i, &price) in prices.iter().enumerate() {
            let mut cur_on_hold : Vec<i32> = on_hold.clone();
            let mut cur_no_hold : Vec<i32> = no_hold.clone();
            let max_buy_times = k;
            for j in 1..=max_buy_times {
                if no_hold[j-1] == i32_min {
                    // otherwise, no_hold[j-1] - price will overflow. 
                    cur_on_hold[j] = on_hold[j];
                } else {
                    cur_on_hold[j] = std::cmp::max(on_hold[j], no_hold[j-1] - price);
                }
            }
            let max_sell_times = k;
            for j in 1..=max_sell_times {
                if on_hold[j] == i32_min {
                    cur_no_hold[j] = no_hold[j];
                } else {
                    cur_no_hold[j] = std::cmp::max(no_hold[j], on_hold[j] + price);
                }
            }

            on_hold = cur_on_hold;
            no_hold = cur_no_hold;
        }
        *no_hold.iter().max().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_188() {
        assert_eq!(Solution::max_profit(2, vec![2,4,1]), 2);
        assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
        assert_eq!(Solution::max_profit(4, vec![1,2,4,2,5,7,2,4,9,0]), 15);
    }
}
