/**
 * [309] Best Time to Buy and Sell Stock with Cooldown
 *
 * You are given an array prices where prices[i] is the price of a given stock on the i^th day.
 * Find the maximum profit you can achieve. You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times) with the following restrictions:
 * 
 * 	After you sell your stock, you cannot buy stock on the next day (i.e., cooldown one day).
 * 
 * Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
 *  
 * Example 1:
 * 
 * Input: prices = [1,2,3,0,2]
 * Output: 3
 * Explanation: transactions = [buy, sell, cooldown, buy, sell]
 * 
 * Example 2:
 * 
 * Input: prices = [1]
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	1 <= prices.length <= 5000
 * 	0 <= prices[i] <= 1000
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // max_profits[i] when holding the stock at i-th day 
        let mut profits_hold = vec![0;prices.len()];
        // max_profits[i] when not holding the stock at i-th day and not selling at (i-1) day. 
        let mut profits_empty = vec![0;prices.len()];
        // max_profits[i] when selling at i-th day. 
        let mut profits_cool = vec![0;prices.len()];

        profits_hold[0] = -prices[0];
        profits_empty[0] = 0;
        profits_cool[0] = -10000; // this state is invalid at 0, set it min to nullify this effect later. 

        // Refer to the class diagram in https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/discuss/75928/Share-my-DP-solution-(By-State-Machine-Thinking). 
        for i in 1..prices.len() {
            profits_hold[i] = std::cmp::max(profits_hold[i-1], profits_empty[i-1]-prices[i]);
            profits_empty[i] = std::cmp::max(profits_empty[i-1], profits_cool[i-1]);
            profits_cool[i] = profits_hold[i-1] + prices[i];
        }

        std::cmp::max(*profits_empty.last().unwrap(), *profits_cool.last().unwrap())
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_309() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
        assert_eq!(Solution::max_profit(vec![4, 2, 7, 1, 11]), 10);
    }
}
