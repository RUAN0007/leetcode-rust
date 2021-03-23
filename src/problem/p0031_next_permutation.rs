/**
 * [31] Next Permutation
 *
 * Implement next permutation, which rearranges numbers into the lexicographically next greater permutation of numbers.
 * If such an arrangement is not possible, it must rearrange it as the lowest possible order (i.e., sorted in ascending order).
 * The replacement must be <a href="http://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in place</a> and use only constant extra memory.
 *  
 * Example 1:
 * Input: nums = [1,2,3]
 * Output: [1,3,2]
 * Example 2:
 * Input: nums = [3,2,1]
 * Output: [1,2,3]
 * Example 3:
 * Input: nums = [1,1,5]
 * Output: [1,5,1]
 * Example 4:
 * Input: nums = [1]
 * Output: [1]
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 100
 * 	0 <= nums[i] <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/next-permutation/
// discuss: https://leetcode.com/problems/next-permutation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut k= None; // the largest index of the first increments
        for i in (0..nums.len()-1).rev() { 
            if nums[i] < nums[i+1] {
                k = Some(i);
                break
            }
        }
        if let None = k {
            return nums.reverse();
        }

        let mut l  = None; // the largest index that nums[l] > nums[k]
        let k = k.unwrap();
        for i in (k+1..nums.len()).rev() { 
            if nums[k] < nums[i] {
                l = Some(i);
                break
            }
        }
        let l = l.unwrap();
        nums.swap(l, k);
        
        // reverse nums[k+1:]
        let length = nums.len();
        let sub_length = (length - (k + 1)) / 2;
        // println!("k={}, l={}", k, l);
        // println!("nums = {:?}", nums);
        // println!("sub_length = {:?}", sub_length);
        for i in 0..sub_length {
            nums.swap(k+1+i, length-1-i);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_31() {
        let mut vec1 = vec![1, 3, 2];
        Solution::next_permutation(&mut vec1);
        assert_eq!(vec1, vec![2, 1, 3]);

        // let mut vec1 = vec![1, 2, 3, 4, 5];
        // Solution::next_permutation(&mut vec1);
        // assert_eq!(vec1, vec![1, 2, 3, 5, 4]);

        // let mut vec2 = vec![5, 4, 3, 2, 1];
        // Solution::next_permutation(&mut vec2);
        // assert_eq!(vec2, vec![1, 2, 3, 4, 5]);
    }
}
