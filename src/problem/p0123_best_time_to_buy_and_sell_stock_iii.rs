/**
 * [123] Best Time to Buy and Sell Stock III
 *
 * You are given an array prices where prices[i] is the price of a given stock on the i^th day.
 * Find the maximum profit you can achieve. You may complete at most two transactions.
 * Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
 *  
 * Example 1:
 * 
 * Input: prices = [3,3,5,0,0,3,1,4]
 * Output: 6
 * Explanation: Buy on day 4 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
 * Then buy on day 7 (price = 1) and sell on day 8 (price = 4), profit = 4-1 = 3.
 * Example 2:
 * 
 * Input: prices = [1,2,3,4,5]
 * Output: 4
 * Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
 * Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are engaging multiple transactions at the same time. You must sell before buying again.
 * 
 * Example 3:
 * 
 * Input: prices = [7,6,4,3,1]
 * Output: 0
 * Explanation: In this case, no transaction is done, i.e. max profit = 0.
 * 
 * Example 4:
 * 
 * Input: prices = [1]
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	1 <= prices.length <= 10^5
 * 	0 <= prices[i] <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/
// discuss: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut balances : Vec<Vec<i32>> = vec![vec![0;n];3];
        for k in 1..=2 {
            let mut min = prices[0];
            for i in 1..n {
                min = std::cmp::min(min, prices[i]-balances[k-1][i-1]);
                balances[k][i] = std::cmp::max(balances[k][i-1], prices[i] - min);
            }
        }
        balances[2][n-1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_123() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    }
}
