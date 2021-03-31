/**
 * [275] H-Index II
 *
 * Given an array of integers citations where citations[i] is the number of citations a researcher received for their i^th paper and citations is sorted in an ascending order, return compute the researcher's h-index.
 * According to the <a href="https://en.wikipedia.org/wiki/H-index" target="_blank">definition of h-index on Wikipedia</a>: A scientist has an index h if h of their n papers have at least h citations each, and the other n - h papers have no more than h citations each.
 * If there are several possible values for h, the maximum one is taken as the h-index.
 *  
 * Example 1:
 * 
 * Input: citations = [0,1,3,5,6]
 * Output: 3
 * Explanation: [0,1,3,5,6] means the researcher has 5 papers in total and each of them had received 0, 1, 3, 5, 6 citations respectively.
 * Since the researcher has 3 papers with at least 3 citations each and the remaining two with no more than 3 citations each, their h-index is 3.
 * 
 * Example 2:
 * 
 * Input: citations = [1,2,100]
 * Output: 2
 * 
 *  
 * Constraints:
 * 
 * 	n == citations.length
 * 	1 <= n <= 10^5
 * 	0 <= citations[i] <= 1000
 * 	citations is sorted in ascending order.
 * 
 *  
 * Follow up: Could you solve it in logarithmic time complexity?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/h-index-ii/
// discuss: https://leetcode.com/problems/h-index-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn helper(citations: &Vec<i32>, start: usize, end: usize) -> i32 {
        // println!("start={}, end={}", start, end);
        // Base case, the number of elements is <= 2. 
        let length = end - start;
        if length <= 2 {
            for i in start..end {
                // println!("   citations.len() - i={}, citations[i]={}", citations.len() - i, citations[i]);
                if citations.len() - i <= citations[i] as usize {
                    return (citations.len() - i) as i32;
                }
            }  // end for
            return 0;
        }  // end if

        let mid = (start + end) / 2;
        if citations.len() - mid <= citations[mid] as usize {
            return Self::helper(citations, start, mid+1);
        } else {
            return Self::helper(citations, mid+1, end);
        }

    }

    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        if n == 0 {return 0}
        let mut low = 0;
        let mut high = citations.len() - 1;
        while low <= high {
            let mid = (low + high) / 2;
            if n - mid == citations[mid] as usize {
                return (n - mid) as i32;
            } else if n - mid <= citations[mid] as usize {
                if mid == 0 || (citations[mid-1] as usize) < (n - mid + 1) {
                    return (n - mid) as i32;
                }

                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        0
    }

    pub fn h_index_2(citations: Vec<i32>) -> i32 {
        // if citations.len() == 0 {
        //     return 0
        // } else {
        //     return Self::helper(&citations, 0, citations.len());
        // }
        let mut low = 0;
        let mut high = citations.len();
        let n = citations.len();
        while low < high {
            let mid = (low + high) / 2;
            if n - mid == citations[mid] as usize {
                return (n - mid) as i32;
            } else if n - mid < citations[mid] as usize {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        return (n - low) as i32;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_275() {
        assert_eq!(Solution::h_index(vec![]), 0);
        assert_eq!(Solution::h_index(vec![0]), 0);
        assert_eq!(Solution::h_index(vec![11, 15]), 2);
        assert_eq!(Solution::h_index(vec![1]), 1);
        assert_eq!(Solution::h_index(vec![0, 1, 3, 5, 6]), 3);
        assert_eq!(Solution::h_index(vec![0, 4, 4, 5, 6]), 4);
        assert_eq!(Solution::h_index(vec![1, 2, 2, 2]), 2);
    }
}
