/**
 * [969] Pancake Sorting
 *
 * Given an array of integers arr, sort the array by performing a series of pancake flips.
 * In one pancake flip we do the following steps:
 * 
 * 	Choose an integer k where 1 <= k <= arr.length.
 * 	Reverse the sub-array arr[0...k-1] (0-indexed).
 * 
 * For example, if arr = [3,2,1,4] and we performed a pancake flip choosing k = 3, we reverse the sub-array [3,2,1], so arr = [<u>1</u>,<u>2</u>,<u>3</u>,4] after the pancake flip at k = 3.
 * Return an array of the k-values corresponding to a sequence of pancake flips that sort arr. Any valid answer that sorts the array within 10 * arr.length flips will be judged as correct.
 *  
 * Example 1:
 * 
 * Input: arr = [3,2,4,1]
 * Output: [4,2,4,3]
 * Explanation: 
 * We perform 4 pancake flips, with k values 4, 2, 4, and 3.
 * Starting state: arr = [3, 2, 4, 1]
 * After 1st flip (k = 4): arr = [<u>1</u>, <u>4</u>, <u>2</u>, <u>3</u>]
 * After 2nd flip (k = 2): arr = [<u>4</u>, <u>1</u>, 2, 3]
 * After 3rd flip (k = 4): arr = [<u>3</u>, <u>2</u>, <u>1</u>, <u>4</u>]
 * After 4th flip (k = 3): arr = [<u>1</u>, <u>2</u>, <u>3</u>, 4], which is sorted.
 * 
 * Example 2:
 * 
 * Input: arr = [1,2,3]
 * Output: []
 * Explanation: The input is already sorted, so there is no need to flip anything.
 * Note that other answers, such as [3, 3], would also be accepted.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= arr.length <= 100
 * 	1 <= arr[i] <= arr.length
 * 	All integers in arr are unique (i.e. arr is a permutation of the integers from 1 to arr.length).
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/pancake-sorting/
// discuss: https://leetcode.com/problems/pancake-sorting/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {

    pub fn reverse_at(arr: &mut Vec<i32>, first_n: usize) {
        for i in 0..(first_n/2) {
            arr.swap(i, first_n - i-1);
        }
    }

    pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let n = arr.len();
        for i in (1..n).rev() {
            let mut max_num = 0;
            let mut max_pos = 0;
            for j in 0..=i {
                if max_num < arr[j] {
                    max_num = arr[j];
                    max_pos = j;
                }
            }
            Self::reverse_at(&mut arr, max_pos+1);
            Self::reverse_at(&mut arr, i+1);
            result.push((max_pos + 1) as i32);
            result.push((i+1) as i32);

            println!("{}, {}, arr={:?}", max_pos+1, i+1, arr);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_969() {
        Solution::pancake_sort(vec![3,2,4,1]);
        // for _i in 0..20 {
        //     let mut rng = rand::thread_rng();
        //     let size = rng.gen_range(0, 1000);
        //     let sorted_vector = make_sorted_vector(size);
        //     let mut shuffled_vector = make_shuffled_vector(&sorted_vector);
        //     let res = Solution::pancake_sort(shuffled_vector.clone());
        //     let oper_num = res.len();
        //     apply_pancake_sort_res(&mut shuffled_vector, res);
        //     assert_eq!(shuffled_vector, sorted_vector);
        //     assert!(oper_num < (size * 10) as usize);
        // }
    }

    fn make_sorted_vector(i: i32) -> Vec<i32> {
        (1..i + 1).collect()
    }

    fn make_shuffled_vector(a: &Vec<i32>) -> Vec<i32> {
        let mut rng = thread_rng();
        let mut res = a.clone();
        res.shuffle(&mut rng);
        res
    }

    fn apply_pancake_sort_res(shuffled_vecter: &mut Vec<i32>, oper: Vec<i32>) {
        for i in oper {
            pancake_oper(shuffled_vecter, (i - 1) as usize);
        }
    }

    pub fn pancake_oper(a: &mut Vec<i32>, index: usize) {
        let mut helper = Vec::new();
        for i in 0..(index + 1) {
            helper.push(a[index - i]);
        }
        for i in 0..(index + 1) {
            a[i] = helper[i];
        }
    }
}
