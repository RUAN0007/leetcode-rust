pub struct Solution {}

// problem: https://leetcode.com/problems/find-right-interval/
// discuss: https://leetcode.com/problems/find-right-interval/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::{collections::HashMap, hash::Hash};
use std::collections::BTreeMap;
use std::collections::HashSet;

struct UnionFind<T :Eq + Hash + Copy> {
    parents: HashMap<T,T>,
    subset_sizes: HashMap<T, usize>,
    pub max_subset_size: usize,
    pub subset_count: usize,
}

impl<T : Eq + Hash + Copy> UnionFind<T> {
    fn new() -> UnionFind<T> {
        let mut uf = UnionFind{
            parents: HashMap::new(), 
            subset_sizes: HashMap::new(),
            max_subset_size: 0,
            subset_count: 0};
        uf
    }

    fn new_with(elements : HashSet<T>) -> UnionFind<T> {
        let mut uf = UnionFind{
            parents: HashMap::new(), 
            subset_sizes: HashMap::new(),
            max_subset_size: 1,
            subset_count: elements.len()};
        
        for &e in &elements {
            uf.parents.insert(e,e);
            uf.subset_sizes.insert(e,1);
        }

        uf
    }
    // None if element is not found. 
    fn find(&self, element : T) -> Option<T> {
        if !self.parents.contains_key(&element) {
            return None;
        }

        let mut root = element;
        while root != *self.parents.get(&root).unwrap() {
            root = *self.parents.get(&root).unwrap();
        }
        Some(root)
    }

    fn find_along_compression(&mut self, element : T) -> Option<T> {
        if let Some(root) = self.find(element) {
            // path compression: redirects each node in the path to the root. 
            let mut element = element;
            while element != *self.parents.get(&element).unwrap() {
                let tmp = *self.parents.get(&element).unwrap();
                *self.parents.get_mut(&element).unwrap() = root;
                element = tmp;
            }

            Some(root)
        } else {
            None
        }
    }

    // return whether the union has performed.     
    fn union(&mut self, e1 : T, e2 : T ) -> bool {
        let root1 = self.find_along_compression(e1);
        let root2 = self.find_along_compression(e2);

        if root1.is_none() {
            // assume to insert this e1 and then do union
            self.parents.insert(e1,e1);
            self.subset_sizes.insert(e1,1);
            self.subset_count+=1;
        } 

        if root2.is_none() {
            // assume to insert this e1 and then do union
            self.parents.insert(e2,e2);
            self.subset_sizes.insert(e2,1);
            self.subset_count+=1;
        }

        let root1= root1.unwrap_or(e1);
        let root2= root2.unwrap_or(e2);
        if root1 == root2 {return false;}

        let root1_size = *self.subset_sizes.get(&root1).unwrap();
        let root2_size = *self.subset_sizes.get(&root2).unwrap();
        // concat the smaller tress to the larger
        if root1_size < root2_size {
            *self.parents.get_mut(&root1).unwrap() = root2;
            *self.subset_sizes.get_mut(&root2).unwrap() += root1_size;
        } else {
            *self.parents.get_mut(&root2).unwrap() = root1;
            *self.subset_sizes.get_mut(&root1).unwrap() += root2_size;
        }
        self.max_subset_size = std::cmp::max(self.max_subset_size, root1_size + root2_size);
        self.subset_count-=1;
        true
    }
}

impl Solution {
    pub fn first_equal(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0i32;
        let mut high = (nums.len() - 1) as i32;
        while low <= high {
            let mid = (low + high) / 2;
            let mid_num = nums[mid as usize];
            if mid_num < target {
                low = mid + 1;
            } else if nums[mid as usize] > target {
                high = mid - 1;
            } else if 0 < mid && nums[(mid-1) as usize] == mid_num {
                high = mid - 1;
            } else {
                return mid;
            }
        }
        -1
    }

    pub fn last_equal(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0i32;
        let mut high = (nums.len() - 1) as i32;
        while low <= high {
            let mid = (low + high) / 2;
            let mid_num = nums[mid as usize];
            if mid_num < target {
                low = mid + 1;
            } else if nums[mid as usize] > target {
                high = mid - 1; // if mid is usize and mid=0, this wil panic. 
            } else if (mid as usize) < nums.len() - 1 && nums[(mid+1) as usize] == mid_num {
                low = mid + 1;
            } else {
                return mid;
            }
        }
        -1
    }

    pub fn first_gt(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0i32;
        let mut high = (nums.len() - 1) as i32;
        while low <= high {
            let mid = (low + high) / 2;
            let mid_num = nums[mid as usize];
            // println!("low={}, mid={}, high={}, mid_num={}, target={}", low, mid, high, mid_num, target);
            if target < mid_num {
                if mid == 0 || nums[(mid-1) as usize] <= target {
                    return mid;
                }
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        -1
    }

    pub fn first_ge(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0i32;
        let mut high = (nums.len() - 1) as i32;

        while low <= high {
            let mid = (low + high) / 2;
            let mid_num = nums[mid as usize];
            // println!("low={}, mid={}, high={}, mid_num={}, target={}", low, mid, high, mid_num, target);
            if target <= mid_num {
                if mid == 0 || nums[(mid-1) as usize] < target {
                    return mid;
                }
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        -1
    }

    // assert_eq!(Solution::last_lt(vec![1,1,3,3,5,5],2), 2);
    pub fn last_lt(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0i32;
        let mut high = (nums.len() - 1) as i32;
        let l = nums.len() as i32;
        while low <= high {
            let mid = (low + high) / 2;
            let mid_num = nums[mid as usize];
            // println!("low={}, mid={}, high={}, mid_num={}, target={}", low, mid, high, mid_num, target);
            if mid_num < target {
                if mid == l - 1 || target <= nums[(mid+1) as usize] {
                    return mid;
                }
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        -1
    }

    pub fn last_le(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0i32;
        let mut high = (nums.len() - 1) as i32;
        let l = nums.len() as i32;
        while low <= high {
            let mid = (low + high) / 2;
            let mid_num = nums[mid as usize];
            // println!("low={}, mid={}, high={}, mid_num={}, target={}", low, mid, high, mid_num, target);
            if mid_num <= target {
                if mid == l - 1 || target < nums[(mid+1) as usize] {
                    return mid;
                }
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        -1
    }

    pub fn max_heapify(nums: &mut Vec<i32>, max_len: usize, start_pos: usize) {
        let left_pos = 2 * start_pos + 1;
        let right_pos = 2 * (start_pos + 1);

        let mut large_pos = None;
        let mut large_val = nums[start_pos];
        if left_pos < max_len && large_val < nums[left_pos] {
            large_val = nums[left_pos];
            large_pos = Some(left_pos);
        }

        if right_pos < max_len && large_val < nums[right_pos] {
            large_val = nums[right_pos];
            large_pos = Some(right_pos);
        }

        if let Some(large_pos) = large_pos {
            nums.swap(start_pos, large_pos);
            Self::max_heapify(nums, max_len, large_pos);
        }
    }

    pub fn heap_sort(nums: &mut Vec<i32>) {
        let n = nums.len();
        for i in (0..(n/2)).rev() {
            Self::max_heapify(nums, nums.len() , i);
        }

        for i in (0..n).rev() {
            nums.swap(0, i);
            Self::max_heapify(nums, i, 0);
        }
    }

    pub fn zero_right_digits(x : i32, digit_count: usize) -> i32 {
        x & (!0 << digit_count)
    }
    
    pub fn nth_digit(x : i32, n : usize) -> i32 {
        (x >> n) & 1
    }
    // if n-th digit is 1, return 2^n, else 0. 
    // n starts from 0. 
    pub fn nth_digit_power(x : i32, n : usize) -> i32 {
        x & (1 << n)
    }

    pub fn set_nth_digit_one(x: i32, n: usize) -> i32 {
        x | (1 << n)
    }

    pub fn set_nth_digit_zero(x: i32, n: usize) -> i32 {
        x & (!(1 << n))
    }

    // inclusive of nth
    pub fn set_zero_from_nth(x: i32, n: usize) -> i32  {
        x & ((1 << n) - 1)
    }

    // inclusive of nth
    pub fn set_zero_to_nth(x: i32, n: usize) -> i32  {
        x & (!((1 << (n + 1)) - 1))
    }

    // set the least significant bit-1 to 0. 
    pub fn set_lsb1_zero(x: i32) -> i32 {
        x & (x-1)
    }

    // 2^(pos of lsb 1)
    pub fn lsb1_power(x: i32) -> i32 {
        x & -x
    }

    pub fn digit1_count(mut x: i32) -> usize {
        let mut count = 0;
        while x != 0 {
            x = Self::set_lsb1_zero(x);
            count+=1;
        }
        count
    }

    // the largest power of 2 le to x
    pub fn largest_power(mut x : u32) -> u32 {
        // println!("{:#032b}", x);
        x = x | (x>>1);
        // println!("{:#032b}", x);
        x = x | (x>>2);
        // println!("{:#032b}", x);
        x = x | (x>>4);
        // println!("{:#032b}", x);
        x = x | (x>>8);
        // println!("{:#032b}", x);
        x = x | (x>>16);
        // println!("{:#032b}", x);
        x = (x+1)>>1;
        // println!("{:#032b}", x);
        x
    }

    pub fn last_entry_ordered_map(m : &BTreeMap<i32, i32>) -> (&i32, &i32) {
        m.iter().next_back().unwrap()
    }

    // transform a string to a char vec for a constant-time complexity to reference a i-th char. 
    pub fn str_to_char_vec(s : String) -> Vec<char> {
        s.chars().collect()
    }

    pub fn incre_from_default(m: &mut HashMap<i32, usize>, key : i32) {
        *(m.entry(key).or_insert(0))+=1;

    }

    pub fn char2idx(c : char) ->usize {
        let base_idx = 'a' as u8 as usize;
        let c_idx = c as u8 as usize;
        c_idx - base_idx
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {

        assert_eq!(Solution::largest_power(9), 8);
        // assert_eq!(Solution::zero_right_digits(7, 1), 6);
        // assert_eq!(Solution::zero_right_digits(8, 2), 8);
        // assert_eq!(Solution::zero_right_digits(-3, 2), -4);

        // assert_eq!(Solution::nth_digit(-3, 0), 1);
        // assert_eq!(Solution::nth_digit(-3, 1), 0);

        // assert_eq!(Solution::nth_digit_power(-3, 1), 0);
        // assert_eq!(Solution::nth_digit_power(5, 2), 4);

        // assert_eq!(Solution::set_nth_digit_zero(5, 2), 1);
        // assert_eq!(Solution::set_nth_digit_one(5, 1), 7);

        // assert_eq!(Solution::set_zero_from_nth(5, 1), 1);
        // assert_eq!(Solution::set_zero_from_nth(5, 3), 5);
        // assert_eq!(Solution::set_zero_from_nth(-3, 3), 5);
        // assert_eq!(Solution::set_zero_from_nth(-3, 4), 13);

        // assert_eq!(Solution::set_zero_to_nth(5, 0), 4);
        // assert_eq!(Solution::set_zero_to_nth(5, 3), 0);

        // assert_eq!(Solution::set_zero_to_nth(-3, 2), -8);
        // assert_eq!(Solution::set_lsb1_zero(5), 4);
        // assert_eq!(Solution::set_lsb1_zero(6), 4);

        // assert_eq!(Solution::lsb1_power(6), 2);
        // assert_eq!(Solution::lsb1_power(7), 1);
        // assert_eq!(Solution::lsb1_power(8), 8);

        // let mut r = vec![12,11,13,5,6,7];
        // Solution::heap_sort(&mut r);
        // assert_eq!(r, vec![5,6,7,11,12,13]);
        // assert_eq!(Solution::first_equal(vec![1,1,3,3,5,5],0), -1);
        // assert_eq!(Solution::first_equal(vec![1,1,3,3,5,5],1), 0);
        // assert_eq!(Solution::first_equal(vec![1,1,3,3,5,5],2), -1);
        // assert_eq!(Solution::first_equal(vec![1,1,3,3,5,5],3), 2);
        // assert_eq!(Solution::first_equal(vec![1,1,3,3,5,5],5), 4);
        // assert_eq!(Solution::first_equal(vec![1,1,3,3,5,5],7), -1);

        // assert_eq!(Solution::last_equal(vec![1,1,3,3,5,5],0), -1);
        // assert_eq!(Solution::last_equal(vec![1,1,3,3,5,5],1), 1);
        // assert_eq!(Solution::last_equal(vec![1,1,3,3,5,5],2), -1);
        // assert_eq!(Solution::last_equal(vec![1,1,3,3,5,5],3), 3);
        // assert_eq!(Solution::last_equal(vec![1,1,3,3,5,5],5), 5);
        // assert_eq!(Solution::last_equal(vec![1,1,3,3,5,5],7), -1);

        // assert_eq!(Solution::first_gt(vec![1,1,3,3,5,5],0), 0);
        // assert_eq!(Solution::first_gt(vec![1,1,3,3,5,5],1), 2);
        // assert_eq!(Solution::first_gt(vec![1,1,3,3,5,5],2), 2);
        // assert_eq!(Solution::first_gt(vec![1,1,3,3,5,5],3), 4);
        // assert_eq!(Solution::first_gt(vec![1,1,3,3,5,5],5), -1);
        // assert_eq!(Solution::first_gt(vec![1,1,3,3,5,5],7), -1);

        // assert_eq!(Solution::first_ge(vec![1,1,3,3,5,5],0), 0);
        // assert_eq!(Solution::first_ge(vec![1,1,3,3,5,5],1), 0);
        // assert_eq!(Solution::first_ge(vec![1,1,3,3,5,5],2), 2);
        // assert_eq!(Solution::first_ge(vec![1,1,3,3,5,5],3), 2);
        // assert_eq!(Solution::first_ge(vec![1,1,3,3,5,5],5), 4);
        // assert_eq!(Solution::first_ge(vec![1,1,3,3,5,5],7), -1);

        // assert_eq!(Solution::last_lt(vec![1,1,3,3,5,5],0), -1);
        // assert_eq!(Solution::last_lt(vec![1,1,3,3,5,5],1), -1);
        // assert_eq!(Solution::last_lt(vec![1,1,3,3,5,5],2), 1);
        // assert_eq!(Solution::last_lt(vec![1,1,3,3,5,5],3), 1);
        // assert_eq!(Solution::last_lt(vec![1,1,3,3,5,5],5), 3);
        // assert_eq!(Solution::last_lt(vec![1,1,3,3,5,5],7), 5);

        // assert_eq!(Solution::last_le(vec![1,1,3,3,5,5],0), -1);
        // assert_eq!(Solution::last_le(vec![1,1,3,3,5,5],1), 1);
        // assert_eq!(Solution::last_le(vec![1,1,3,3,5,5],2), 1);
        // assert_eq!(Solution::last_le(vec![1,1,3,3,5,5],3), 3);
        // assert_eq!(Solution::last_le(vec![1,1,3,3,5,5],5), 5);
        // assert_eq!(Solution::last_le(vec![1,1,3,3,5,5],7), 5);


        let mut elements = HashSet::new();
        elements.insert(0);
        elements.insert(1);
        elements.insert(2);
        elements.insert(3);
        elements.insert(4);
        let mut uf = UnionFind::new_with(elements);
        assert_eq!(uf.max_subset_size, 1);
        assert_eq!(uf.subset_count, 5);
        assert_eq!(*uf.parents.get(&3).unwrap(), 3);
        assert_eq!(uf.find(3), Some(3));
        assert_eq!(uf.find(6), None);

        // shall result like
        // 0 2 3 4
        // - 
        // 1
        assert!(uf.union(0, 1));
        assert_eq!(uf.max_subset_size, 2);
        assert_eq!(uf.subset_count, 4);
        assert_eq!(*uf.parents.get(&0).unwrap(), 0);
        assert_eq!(*uf.parents.get(&1).unwrap(), 0);
        assert_eq!(uf.find(1), Some(0));
        assert_eq!(uf.find(4), Some(4));

        // shall result like
        // 0 3 4
        // - -
        // 1 2
        assert!(uf.union(3, 2));
        assert_eq!(uf.max_subset_size, 2);
        assert_eq!(uf.subset_count, 3);
        assert_eq!(*uf.parents.get(&2).unwrap(), 3);
        assert_eq!(*uf.parents.get(&3).unwrap(), 3);
        assert_eq!(uf.find(4), Some(4));
        assert_eq!(uf.find(2), Some(3));

        // shall result like
        // 0   3 
        // -  - -
        // 1  2 4
        assert!(uf.union(4, 2));
        assert!(!uf.union(1, 0));
        assert_eq!(uf.max_subset_size, 3);
        assert_eq!(uf.subset_count, 2);
        assert_eq!(*uf.parents.get(&4).unwrap(), 3);
        assert_eq!(*uf.parents.get(&3).unwrap(), 3);
        assert_eq!(*uf.parents.get(&2).unwrap(), 3);

        // shall result like
        //   3 
        // - - -
        // 0 2 4
        // -
        // 1
        assert!(uf.union(1, 4));
        assert_eq!(uf.max_subset_size, 5);
        assert_eq!(uf.subset_count, 1);
        assert_eq!(*uf.parents.get(&1).unwrap(), 0);
        assert_eq!(*uf.parents.get(&0).unwrap(), 3);
        assert_eq!(*uf.parents.get(&2).unwrap(), 3);

        assert_eq!(uf.find(1), Some(3));
        assert_eq!(uf.find(2), Some(3));
        assert_eq!(uf.find(4), Some(3));

        assert_eq!(uf.find_along_compression(1), Some(3));
        // shall result like
        //   3 
        // - - - -
        // 0 1 2 4
        assert_eq!(*uf.parents.get(&1).unwrap(), 3);
        assert_eq!(*uf.parents.get(&0).unwrap(), 3);
        assert_eq!(*uf.parents.get(&2).unwrap(), 3);

        assert!(!uf.union(3, 4));

        // shall result like
        //   3 
        // - - - - -
        // 0 1 2 4 5
        assert!(uf.union(5, 0)); // a new element 5
        assert_eq!(*uf.parents.get(&5).unwrap(), 3);
        assert_eq!(uf.find(5), Some(3));
        assert_eq!(uf.subset_count, 1);
        assert_eq!(uf.max_subset_size, 6);
    }
}
