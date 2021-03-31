/**
 * [274] H-Index
 *
 * Given an array of integers citations where citations[i] is the number of citations a researcher received for their i^th paper, return compute the researcher's h-index.
 * According to the <a href="https://en.wikipedia.org/wiki/H-index" target="_blank">definition of h-index on Wikipedia</a>: A scientist has an index h if h of their n papers have at least h citations each, and the other n - h papers have no more than h citations each.
 * If there are several possible values for h, the maximum one is taken as the h-index.
 *  
 * Example 1:
 * 
 * Input: citations = [3,0,6,1,5]
 * Output: 3
 * Explanation: [3,0,6,1,5] means the researcher has 5 papers in total and each of them had received 3, 0, 6, 1, 5 citations respectively.
 * Since the researcher has 3 papers with at least 3 citations each and the remaining two with no more than 3 citations each, their h-index is 3.
 * 
 * Example 2:
 * 
 * Input: citations = [1,3,1]
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	n == citations.length
 * 	1 <= n <= 5000
 * 	0 <= citations[i] <= 1000
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/h-index/
// discuss: https://leetcode.com/problems/h-index/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::BTreeMap;
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut counts = BTreeMap::new();
        for citation in citations {
            if let Some(count) = counts.get_mut(&citation) {
                *count += 1;
            } else {
                counts.insert(citation, 1);
            }
        }
        let mut count_sum = 0;
        for (&citation, &count) in counts.iter().rev() {
            if count_sum + count > citation {
                count_sum = std::cmp::max(citation, count_sum);
                break;
            }
            count_sum += count;
            // so far, ${count_sum} papers have citation at least ${citation}.
        }
        count_sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_274() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
        assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
        assert_eq!(Solution::h_index(vec![1, 1]), 1);
        assert_eq!(Solution::h_index(vec![0, 0, 4, 4]), 2);
    }
}
