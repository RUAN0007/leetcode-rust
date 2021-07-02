/**
 * [307] Range Sum Query - Mutable
 *
 * Given an integer array nums, handle multiple queries of the following types:
 * <ol>
 * 	Update the value of an element in nums.
 * 	Calculate the sum of the elements of nums between indices left and right inclusive where left <= right.
 * </ol>
 * Implement the NumArray class:
 * 
 * 	NumArray(int[] nums) Initializes the object with the integer array nums.
 * 	void update(int index, int val) Updates the value of nums[index] to be val.
 * 	int sumRange(int left, int right) Returns the sum of the elements of nums between indices left and right inclusive (i.e. nums[left] + nums[left + 1] + ... + nums[right]).
 * 
 *  
 * Example 1:
 * 
 * Input
 * ["NumArray", "sumRange", "update", "sumRange"]
 * [[[1, 3, 5]], [0, 2], [1, 2], [0, 2]]
 * Output
 * [null, 9, null, 8]
 * Explanation
 * NumArray numArray = new NumArray([1, 3, 5]);
 * numArray.sumRange(0, 2); // return 1 + 3 + 5 = 9
 * numArray.update(1, 2);   // nums = [1, 2, 5]
 * numArray.sumRange(0, 2); // return 1 + 2 + 5 = 8
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 3 * 10^4
 * 	-100 <= nums[i] <= 100
 * 	0 <= index < nums.length
 * 	-100 <= val <= 100
 * 	0 <= left <= right < nums.length
 * 	At most 3 * 10^4 calls will be made to update and sumRange.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/range-sum-query-mutable/
// discuss: https://leetcode.com/problems/range-sum-query-mutable/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct NumArray(Vec<i32>);

impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let l : usize = nums.len();
        let mut na : Self = Self(vec![0;l+1]);
        for (i, &num) in nums.iter().enumerate() {
            na.increment(i as i32, num);
        }

        na
    }
    
    fn update(&mut self, index: i32, val: i32) {
       let old = self.get(index);
       let inc : i32 = val - old;
       self.increment(index, inc);
    }

    fn increment(&mut self, index: i32, incr: i32) {
       self.increment_inner(index+1, incr);
    }

    fn sum_query(&self, idx : i32 ) -> i32 {
       self.sum_query_inner(idx+1)
    }
    
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == 0 {
            self.sum_query(right)
        } else {
            self.sum_query(right) - self.sum_query(left-1)
        }
    }

    fn get(&self, idx : i32) -> i32 {
        // self.sum_range(idx, idx)
        self.get_inner(idx+1)
    }

    fn inner(&self) -> Vec<i32> {
        self.0.clone()
    }

    fn get_inner(&self, mut idx : i32) -> i32 {
        let mut sum : i32 = self.0[idx as usize];
        let idx_no_ls1b : i32 = idx & (idx - 1);
        idx-=1;
        while  idx != idx_no_ls1b {
            sum -= self.0[idx as usize];
            idx &= (idx-1);
        }
        sum
    }

    // inner function assumes index starting from 0. 
    fn increment_inner(&mut self, mut index: i32, incr: i32) {
       while (index as usize) < self.0.len() {
           self.0[index as usize] += incr;
           index += index & -index;
       }
    }

    fn sum_query_inner(&self, mut idx : i32 ) -> i32 {
       let mut sum : i32 = 0;
       while 0 < idx {
           sum += self.0[idx as usize];
           idx &=(idx-1);
       }
       sum
    }

    // Assume non-negative elements
    fn last_idx_prefix_sum_le(&self, mut target : i32) -> i32 {
        // the MSB of index. 
        let mut bit_mask : i32 = self.0.len() as i32- 1;
        bit_mask = bit_mask | (bit_mask >> 1);
        bit_mask = bit_mask | (bit_mask >> 2);
        bit_mask = bit_mask | (bit_mask >> 4);
        bit_mask = bit_mask | (bit_mask >> 8);
        bit_mask = bit_mask | (bit_mask >> 16);
        bit_mask = (bit_mask+1)>>1;

        let mut base_idx : usize = 0;
        while bit_mask != 0 {
            let idx : usize = base_idx + bit_mask as usize;
            bit_mask >>= 1;
            if idx >= self.0.len() {continue;}
            if self.0[idx] <= target {
                target -= self.0[idx];
                base_idx = idx;
            }
        }
        if target == 0 {
            // find an equal case
        } else {
            // println!("target={}", target);
            // find an less than case.
        }
        base_idx as i32 - 1
    }

}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_307() {
        let _empty = NumArray::new(vec![]);
        let mut tree = NumArray::new(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(tree.sum_range(0, 6), 7);
        assert_eq!(tree.last_idx_prefix_sum_le(7), 6);
        assert_eq!(tree.last_idx_prefix_sum_le(17), 9);

        tree.update(0, 2);

        assert_eq!(tree.sum_range(0, 6), 8);
        assert_eq!(tree.get(0), 2);

        tree.update(1, 2);
        assert_eq!(tree.last_idx_prefix_sum_le(4), 1);
        assert_eq!(tree.sum_range(0, 2), 5);

        tree.update(6, 10);
        assert_eq!(tree.last_idx_prefix_sum_le(16), 5);
        assert_eq!(tree.last_idx_prefix_sum_le(18), 6);
        assert_eq!(tree.sum_range(6, 6), 10);
    }
}
