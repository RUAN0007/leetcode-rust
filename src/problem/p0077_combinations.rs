/**
 * [77] Combinations
 *
 * Given two integers n and k, return all possible combinations of k numbers out of 1 ... n.
 * You may return the answer in any order.
 *  
 * Example 1:
 * 
 * Input: n = 4, k = 2
 * Output:
 * [
 *   [2,4],
 *   [3,4],
 *   [2,3],
 *   [1,2],
 *   [1,3],
 *   [1,4],
 * ]
 * 
 * Example 2:
 * 
 * Input: n = 1, k = 1
 * Output: [[1]]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 20
 * 	1 <= k <= n
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/combinations/
// discuss: https://leetcode.com/problems/combinations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn helper(result: &mut Vec<Vec<i32>>, tmp: &mut Vec<i32>, start:i32, n: i32, k: usize) {
        // println!("start={}, n={}, k={}, tmp={:?}", start, n, k, tmp);
        if k == 0 {
            result.push(tmp.clone());
            return;
        }
        for i in start..=n {
            tmp.push(i);
            Self::helper(result, tmp, i + 1, n, k-1);
            tmp.pop();
        }
        // println!("===============================================");
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut tmp = vec![];
        Self::helper(&mut result, &mut tmp, 1, n, k as usize);
        result
    }


    pub fn combine_dp(n: i32, k: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let k = k as usize;
        let mut all : Vec<Vec<Vec<i32>>> = vec![vec![vec![]]; k+1];

        for ni in 1..=n {
            // (1..i as i32).collect();
            all[0] = vec![vec![]];
            for ki in (1..=k as usize).rev() {
                if ki == ni {
                    all[ni] = vec![(1..=ni as i32).collect()];

                } else if ki < ni {
                    // println!("\tki={}, ni={}, all[ki-1] = {:?}", ki, ni, all[ki-1]);
                    for mut prev_comb in all[ki-1].clone() {
                        prev_comb.push(ni as i32);
                        // println!("\tki={}, ni={}, prev_comb = {:?}", ki, ni, prev_comb);
                        all[ki].push(prev_comb);
                    }

                } else {
                    // n < ki => invalid case, ignore. 
                }
            }
            // println!("ni={}, {:?}", ni, all);
        }
        all[k].clone()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_77() {
        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ]
        );
        assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::combine(0, 1), empty);
        assert_eq!(Solution::combine(2, 1), vec![vec![1], vec![2]]);
    }
}
