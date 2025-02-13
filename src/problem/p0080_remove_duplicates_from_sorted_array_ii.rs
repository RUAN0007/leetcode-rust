/**
 * [80] Remove Duplicates from Sorted Array II
 *
 * Given a sorted array nums, remove the duplicates <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> such that duplicates appeared at most twice and return the new length.
 * Do not allocate extra space for another array; you must do this by modifying the input array <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> with O(1) extra memory.
 * Clarification:
 * Confused why the returned value is an integer, but your answer is an array?
 * Note that the input array is passed in by reference, which means a modification to the input array will be known to the caller.
 * Internally you can think of this:
 * 
 * // nums is passed in by reference. (i.e., without making a copy)
 * int len = removeDuplicates(nums);
 * // any modification to nums in your function would be known by the caller.
 * // using the length returned by your function, it prints the first len elements.
 * for (int i = 0; i < len; i++) {
 *     print(nums[i]);
 * }
 * 
 *  
 * Example 1:
 * 
 * Input: nums = [1,1,1,2,2,3]
 * Output: 5, nums = [1,1,2,2,3]
 * Explanation: Your function should return length = 5, with the first five elements of nums being 1, 1, 2, 2 and 3 respectively. It doesn't matter what you leave beyond the returned length.
 * 
 * Example 2:
 * 
 * Input: nums = [0,0,1,1,1,1,2,3,3]
 * Output: 7, nums = [0,0,1,1,2,3,3]
 * Explanation: Your function should return length = 7, with the first seven elements of nums being modified to 0, 0, 1, 1, 2, 3 and 3 respectively. It doesn't matter what values are set beyond the returned length.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 3 * 10^4
 * 	-10^4 <= nums[i] <= 10^4
 * 	nums is sorted in ascending order.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
// discuss: https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let l : usize = nums.len();
        let mut i : usize = 0;
        let mut pre_inserted_num : i32 = -100000; // any num not in nums
        let mut pre_inserted_count : usize = 2;
        let mut insert_pos : usize = 0;
        while i < l {
            if nums[i] == pre_inserted_num {
                if pre_inserted_count == 1 {
                    nums[insert_pos] = nums[i];
                    insert_pos +=1;
                    pre_inserted_count = 2; 
                } else {
                    // do nothing as the pre_inserted_num is included twice
                }
            } else {
                nums[insert_pos] = nums[i];
                insert_pos +=1;
                pre_inserted_num = nums[i];
                pre_inserted_count = 1;
            }
            i+=1;
        }
        insert_pos as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_80() {
        let mut nums : Vec<i32> = vec![1,1,1,2,2,3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums[..5], vec![1,1,2,2,3]);

        let mut nums : Vec<i32> = vec![0,0,1,1,1,1,2,3,3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 7);
        assert_eq!(nums[..7], vec![0,0,1,1,2,3,3]);
    }
}
