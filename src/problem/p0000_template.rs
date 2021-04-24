pub struct Solution {}

// problem: https://leetcode.com/problems/find-right-interval/
// discuss: https://leetcode.com/problems/find-right-interval/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::{collections::HashMap, hash::Hash};
use std::collections::BTreeMap;
use std::collections::HashSet;


pub fn left_rightmost_smaller_idx(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();

    let mut left_rightmost_smaller_idx = vec![-1i32;n];
    let mut stack : Vec<usize> = vec![];
    for (i, &num) in nums.iter().enumerate() {
        while let Some(&last_idx) = stack.last() {
            if !(nums[last_idx] < num) {
                stack.pop();
            } else {
                break;
            }
        }

        if let Some(&last_idx) = stack.last() {
            left_rightmost_smaller_idx[i] = last_idx as i32;
        } else {
            left_rightmost_smaller_idx[i] = -1;
        }
        stack.push(i);
    }
    left_rightmost_smaller_idx
}

struct UnionFindInt {
    parents: Vec<usize>,
    subset_sizes: Vec<usize>,
    pub max_subset_size: usize,
    pub subset_count: usize,
}

impl UnionFindInt {
    fn new(element_count : usize) -> UnionFindInt {
        let mut uf = UnionFindInt{
            parents: (0..element_count).collect(),
            subset_sizes: vec![1;element_count],
            max_subset_size: 1,
            subset_count: element_count};
        uf
    }

    fn find_root(&self, element_id : usize) -> usize {
        let mut root = element_id;
        while root != self.parents[root] {
            root = self.parents[root];
        }
        root
    }

    fn find_root_with_compression(&mut self, element_id : usize) -> usize {
        let root = self.find_root(element_id);

        // path compression
        let mut element_id = element_id;
        while self.parents[element_id] != root {
            let tmp = self.parents[element_id];
            self.parents[element_id] = root;
            element_id = tmp;
        }

        root
    }

    fn union(&mut self, e1 : usize, e2 : usize) -> bool {
        let root1 = self.find_root_with_compression(e1);
        let root2 = self.find_root_with_compression(e2);
        if root1 == root2 { return false; }

        if self.subset_sizes[root1] < self.subset_sizes[root2] {
            // anchor root1 tree under root2 tree
            self.parents[root1] = root2;
            self.subset_sizes[root2] += self.subset_sizes[root1];
            self.max_subset_size = std::cmp::max(self.max_subset_size, self.subset_sizes[root2]);
        } else {
            self.parents[root2] = root1;
            self.subset_sizes[root1] += self.subset_sizes[root2];
            self.max_subset_size = std::cmp::max(self.max_subset_size, self.subset_sizes[root1]);
        }
        self.subset_count -=1;
        true
    }
}


struct UnionFind<T :Eq + Hash + Clone> {
    parents: HashMap<T,T>,
    subset_sizes: HashMap<T, usize>,
    pub max_subset_size: usize,
    pub subset_count: usize,
}

impl<T : Eq + Hash + Clone> UnionFind<T> {
    fn new() -> UnionFind<T> {
        let mut uf = UnionFind{
            parents: HashMap::new(), 
            subset_sizes: HashMap::new(),
            max_subset_size: 0,
            subset_count: 0};
        uf
    }

    fn new_with(elements : &HashSet<T>) -> UnionFind<T> {
        let mut uf = UnionFind{
            parents: HashMap::new(), 
            subset_sizes: HashMap::new(),
            max_subset_size: 1,
            subset_count: elements.len()};
        
        for e in elements {
            uf.parents.insert(e.clone(),e.clone());
            uf.subset_sizes.insert(e.clone(),1);
        }

        uf
    }
    // None if element is not found. 
    fn find(&self, element : &T) -> Option<T> {
        if !self.parents.contains_key(element) {
            return None;
        }

        let mut root = element.clone();
        while root != *self.parents.get(&root).unwrap() {
            root = self.parents.get(&root).unwrap().clone();
        }
        Some(root)
    }

    fn find_along_compression(&mut self, element : &T) -> Option<T> {
        if let Some(root) = self.find(element) {
            // path compression: redirects each node in the path to the root. 
            let mut element = element.clone();
            while element != *self.parents.get(&element).unwrap() {
                let tmp = self.parents[&element].clone();
                *self.parents.get_mut(&element).unwrap() = root.clone();
                element = tmp;
            }

            Some(root)
        } else {
            None
        }
    }

    // return whether the union has performed.     
    fn union(&mut self, e1 : &T, e2 : &T ) -> bool {
        let root1 = self.find_along_compression(e1);
        let root2 = self.find_along_compression(e2);

        if root1.is_none() {
            // assume to insert this e1 and then do union
            self.parents.insert(e1.clone(),e1.clone());
            self.subset_sizes.insert(e1.clone(),1);
            self.subset_count+=1;
        } 

        if root2.is_none() {
            // assume to insert this e1 and then do union
            self.parents.insert(e2.clone(),e2.clone());
            self.subset_sizes.insert(e2.clone(),1);
            self.subset_count+=1;
        }

        let root1= root1.unwrap_or(e1.clone());
        let root2= root2.unwrap_or(e2.clone());
        if root1 == root2 {return false;}

        let root1_size = *self.subset_sizes.get(&root1).unwrap();
        let root2_size = *self.subset_sizes.get(&root2).unwrap();
        // concat the smaller tress to the larger
        if root1_size < root2_size {
            *self.parents.get_mut(&root1).unwrap() = root2.clone();
            *self.subset_sizes.get_mut(&root2).unwrap() += root1_size;
        } else {
            *self.parents.get_mut(&root2).unwrap() = root1.clone();
            *self.subset_sizes.get_mut(&root1).unwrap() += root2_size;
        }
        self.max_subset_size = std::cmp::max(self.max_subset_size, root1_size + root2_size);
        self.subset_count-=1;
        true
    }
}

struct BinarySearch {}

impl BinarySearch {
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
}

// Use vector to represent a heap so that any element can be randomly accessed, but the element count must be fixed. 
use  std::cmp::Ordering;
#[derive(Debug)]
struct VecHeap<K: Clone + Hash + Eq, W:Ord + Clone, V: Clone>{
    elements: Vec<(K,W,V)>,
    key2idx: HashMap<K, usize>,
} 

impl<K: Clone + Hash + Eq, W:Ord + Clone, V: Clone> VecHeap<K,W,V>{

    pub fn new(keys: Vec<K>, weights : Vec<W>, values : Vec<V>) -> VecHeap<K,W,V> {
        let mut vh = VecHeap{elements: vec![], key2idx: HashMap::new()};
        let n = keys.len();
        for i in 0..keys.len() {
            let idx = vh.elements.len();
            vh.key2idx.insert(keys[i].clone(), idx);
            vh.elements.push((keys[i].clone(), weights[i].clone(), values[i].clone()));
        }

        for i in (0..(n/2)).rev() {
            vh.topdown_heapify(i, n);
        }

        vh
    }

    pub fn reweight_with_default(&mut self, key: &K, weight: &W, default_value: V) -> bool {
        if let Some((k,w,v)) = self.remove(key) {
            self.insert(k, weight.clone(),v);
            true
        } else {
            self.insert(key.clone(), weight.clone(),default_value);
            false
        }
    }

    pub fn reweight(&mut self, key: &K, weight: &W) -> bool {
        if let Some((k,w,v)) = self.remove(key) {
            self.insert(k, weight.clone(),v);
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, key : &K) -> Option<(K,W,V)> {
        if let Some(&removed_pos) = self.key2idx.get(key) {
            // swap the removed element with the last element. 
            self.key2idx.remove(key);
            if removed_pos == self.elements.len() - 1 {
                self.elements.pop()
            } else  {
                //swap the removed element wit the last. 
                let removed = self.elements[removed_pos].clone();
                let last_entry = self.elements.pop().unwrap();
                self.key2idx.insert(last_entry.0.clone(), removed_pos);
                self.elements[removed_pos] = last_entry;

                // topdown_heapify from the removed pos
                self.topdown_heapify(removed_pos, self.len());
                Some(removed)
            }

        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, weight: W, value: V) {
        let last_pos = self.elements.len();
        self.key2idx.insert(key.clone(), last_pos);
        self.elements.push((key, weight, value));
        self.bottomup_heapify(last_pos)
    }

    pub fn bottomup_heapify(&mut self, start_pos : usize) {
        if 0 < start_pos  {
            let parent_pos = (start_pos + 1) / 2 - 1;
            if self.elements[parent_pos].1.cmp(&self.elements[start_pos].1) == Ordering::Less {
                self.elements.swap(parent_pos, start_pos);
                self.key2idx.insert(self.elements[start_pos].0.clone(), start_pos);
                self.key2idx.insert(self.elements[parent_pos].0.clone(), parent_pos);
                self.bottomup_heapify(parent_pos);
            }
        }
    }
    
    pub fn topdown_heapify(&mut self, start_pos: usize, max_len: usize ) {
        let left_pos = 2 * start_pos + 1;
        let right_pos = 2 * (start_pos + 1);

        let mut large_pos = None;
        let mut large_weight = self.elements[start_pos].1.clone();
        if left_pos < max_len && large_weight.cmp(&self.elements[left_pos].1) == Ordering::Less {
            large_weight = self.elements[left_pos].1.clone();
            large_pos = Some(left_pos);
        }

        if right_pos < max_len && large_weight.cmp(&self.elements[right_pos].1) == Ordering::Less {
            large_weight = self.elements[right_pos].1.clone();
            large_pos = Some(right_pos);
        }

        if let Some(large_pos) = large_pos {
            self.elements.swap(start_pos, large_pos);
            self.key2idx.insert(self.elements[start_pos].0.clone(), start_pos);
            self.key2idx.insert(self.elements[large_pos].0.clone(), large_pos);

            self.topdown_heapify(large_pos, max_len);
        }
    }

    pub fn max(&self) -> Option<&(K,W,V)> {
        self.elements.get(0)
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

}

use core::fmt::Debug;
pub fn heap_sort<T: Clone + Hash + Ord + Debug> (nums: &mut Vec<T>) {
    let n = nums.len();
    let mut vh = VecHeap::new(nums.clone(), 
    nums.clone(), nums.clone());
    // println!("vh.elements={:?}", vh.elements);
    // println!("vh.key2idx={:?}", vh.key2idx);

    for i in (0..n).rev() {
        vh.elements.swap(0, i);
        vh.topdown_heapify(0, i);
    }
    nums.clear();
    for entry in vh.elements {
        nums.push(entry.0);
    }
}


struct BitOp{} 
impl BitOp {
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
}
#[derive(Debug, Clone, Copy)]
struct StrUtil{} 
impl StrUtil {
    // transform a string to a char vec for a constant-time complexity to reference a i-th char. 
    pub fn str_to_char_vec(s : String) -> Vec<char> {
        s.chars().collect()
    }

    pub fn char2idx(c : char) ->usize {
        let base_idx = 'a' as u8 as usize;
        let c_idx = c as u8 as usize;
        c_idx - base_idx
    }

    pub fn str2int(s : String) -> i32 {
        s.parse::<i32>().unwrap()
    }

    pub fn int2str(i : i32) -> String {
        i.to_string()
    }

    pub fn misc(s : String) {
        let s : & str = &s[..];
        let sub : String = s[1..2].to_owned();

        let mut hello = String::from("Hello, ");
        hello.push('w'); // concat char
        hello.push_str("orld!"); // append str

        let sentence: &'static str = "the quick brown fox jumps over the lazy dog";

        // word typed as &str
        for word  in sentence.split_whitespace().rev() {
            // println!("> {}", word);
        }

        // Heap allocate a string
        let alice = String::from("I like dogs");
        // Allocate new memory and store the modified string there
        let bob: String = alice.replace("dog", "cat");

    }
}

struct TreeUtil {}
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use crate::util::tree::{TreeNode, to_tree};

impl TreeUtil {
    pub fn bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut traversed : Vec<i32> = vec![];
        if let Some(ref root_node) = root {
            type NodeWithLevel = (Rc<RefCell<TreeNode>>, usize);
            let mut queue : VecDeque<NodeWithLevel> = VecDeque::new();
            queue.push_back((Rc::clone(root_node), 1));

            while let Some(head_entry) = queue.pop_front() { 
                let cur_node : Rc<RefCell<TreeNode>> = head_entry.0;
                let cur_level : usize = head_entry.1;
                traversed.push(cur_node.borrow().val);
                // left_node typed with &Rc<RefCell<TreeNode>>
                if let Some(left_node) = cur_node.borrow().left.as_ref() {
                    queue.push_back((Rc::clone(left_node), cur_level+1));
                };

                // right_node typed with &Rc<RefCell<TreeNode>>
                if let Some(right_node) = cur_node.borrow().right.as_ref() {
                    queue.push_back((Rc::clone(right_node), cur_level+1));
                };
            }
        }

        traversed
    }

    pub fn preorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_some() {
            let mut r = vec![root.as_ref().unwrap().borrow().val];
            r.extend(Self::preorder(&root.as_ref().unwrap().borrow().left));
            r.extend(Self::preorder(&root.as_ref().unwrap().borrow().right));
            r
        } else {
            vec![]
        }
    }

    pub fn inorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_some() {
            let mut r = vec![];
            r.extend(Self::preorder(&root.as_ref().unwrap().borrow().left));
            r.push(root.as_ref().unwrap().borrow().val);
            r.extend(Self::preorder(&root.as_ref().unwrap().borrow().right));
            r
        } else {
            vec![]
        }
    }

    pub fn postorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_some() {
            let mut r = vec![];
            r.extend(Self::preorder(&root.as_ref().unwrap().borrow().left));
            r.extend(Self::preorder(&root.as_ref().unwrap().borrow().right));
            r.push(root.as_ref().unwrap().borrow().val);
            r
        } else {
            vec![]
        }
    }

    pub fn vec2bst_helper(nums : &[i32], size: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if size <= 0 {
            None
        } else {
            // biase to left half so that mid will not overflow. 
            let mid = size / 2;
            let left_size = mid;
            let right_size = size - 1 - mid;
            let node = Rc::new(RefCell::new(TreeNode::new(nums[mid as usize])));
            if 0 < mid {
                node.borrow_mut().left = Self::vec2bst_helper(&nums[0..(mid as usize)], left_size);
            }

            if mid+1 < size {
                node.borrow_mut().right = Self::vec2bst_helper(&nums[(mid+1) as usize..], right_size);
            }
            Some(node)
        }
    }

    pub fn vec2bst(nums : Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::vec2bst_helper(&nums[..], nums.len() as i32)
    }
}


struct VecUtil{} 
impl VecUtil {
    pub fn sort(v : &mut Vec<i32>) {
       v.sort_by(|a : &i32,b: &i32|{a.cmp(b)});
    }

    pub fn range2vec(from_inclusive: i32, to_exclusive : i32) -> Vec<i32> {
        (from_inclusive..to_exclusive).collect()
    }

    pub fn incre_foreach(v : &mut Vec<i32>) {
        v.iter_mut().for_each(|x : &mut i32|{*x+=1})
    }

    pub fn incre_foreach_to_vec(v: &Vec<i32>) -> Vec<i32> {
        v.iter().map(|x : &i32|{x+1}).collect()
    }

    pub fn stack() {
        let mut s = vec![];
        s.push(0);
        let e : Option<i32> = s.pop();
    }

    pub fn queue() {
        use std::collections::VecDeque;
        let mut q: VecDeque<i32> = VecDeque::new();
        q.push_back(0);
        let e : Option<i32> = q.pop_front();
    }

    pub fn iterator_misc() {
        let mut xs = vec![0;100];
        let processed : Vec<i32> = xs.into_iter()
            .skip(5) // skip the previous 5 elements
            .filter(|x : &i32|{*x < 50}) // filter by a predicate
            .take(10).collect(); // only consider the first 10.
        let e1 : Option<&i32> = processed.iter().next();
        let e10 : Option<&i32> = processed.iter().nth(10);
        let last : Option<&i32> = processed.iter().last();
        let sum : i32 = processed.iter().sum();
        let count : usize = processed.iter().count();

        let a1 = [1, 2, 3];
        let a2 = [4, 5, 6];

        let mut iter = a1.iter().zip(a2.iter());

        assert_eq!(iter.next(), Some((&1, &4)));
        assert_eq!(iter.next(), Some((&2, &5)));
        assert_eq!(iter.next(), Some((&3, &6)));
        assert_eq!(iter.next(), None);


        let a = ["1", "two", "NaN", "four", "5"];
        // Filter_map: The returned iterator yields only the values for which the supplied closure returns Some(value).
        let mut iter = a.iter().filter_map(|s| s.parse().ok());

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);

        let words = ["alpha", "beta", "gamma"];
        // Flat_map: Creates an iterator that works like map, but flattens nested structure.
        let merged: String = words.iter()
                                .flat_map(|s| s.chars())
                                .collect();
        assert_eq!(merged, "alphabetagamma".to_owned());

                                let a = [1, 2, 3];

        let (even, odd): (Vec<i32>, Vec<i32>) = a
            .iter()
            .partition(|&n| n % 2 == 0);

        assert_eq!(even, vec![2]);
        assert_eq!(odd, vec![1, 3]);

        // Fold
        let a = [1, 2, 3];
        let sum = a.iter().fold(0, |acc : i32, x : &i32| {acc + x});
        assert_eq!(sum, 6);

        // All/any: Tests if every/any element of the iterator matches a predicate.
        let a = [1, 2, 3];
        assert!(a.iter().all(|x : &i32| *x > 0));
        assert!(a.iter().any(|x : &i32| *x > 0));
        
        // Find: find an element idx from left
        // position/rposition: right the element index
        let a = [1, 2, 3, 2];
        assert_eq!(a.iter().find(|x : &&i32| **x == 2), Some(&2));
        assert_eq!(a.iter().position(|x : &i32| *x == 2), Some(1usize));
        assert_eq!(a.iter().rposition(|x : &i32| *x == 2), Some(3usize));

        let m : Option<&i32> = a.iter().max(); // min or max_by_key, max_by
        assert_eq!(m, Some(&3));
    }

    pub fn misc() {
        let mut xs = vec![1i32, 2, 3];

        // Insert new element at the end of the vector
        xs.push(4);
        assert_eq!(xs, vec![1,2,3,4]);

        // The `len` method yields the number of elements currently stored in a vector
        assert_eq!(xs.len(), 4);

        // Indexing is done using the square brackets (indexing starts at 0)
        assert_eq!(xs[1], 2);

        // `pop` removes the last element from the vector and returns it
        assert_eq!(xs.pop(), Some(4));

        // `Vector`s can be easily iterated over
        // println!("Contents of xs:");
        // x typed as &i32
        for x in xs.iter() {
            // println!("> {}", x);
        }

        // x typed as &mut i32
        for x in xs.iter_mut() {
            *x +=1;
        }

        // A `Vector` can also be iterated over while the iteration
        // count is enumerated in a separate variable (`i`)
        // i typed as usize. x typed as &i32
        for (i, x) in xs.iter().enumerate() {
            // println!("In position {} we have value {}", i, x);
        }
    }
}

struct MapUtil{} 
impl MapUtil {
    pub fn hashmap_misc() {
        let mut map : HashMap<String, String> = HashMap::new();
        let k1 = String::from("k1");
        let v1 = String::from("v1");
        let prev_val : Option<String> =map.insert(k1, v1); // k1, v1 is consumed here.
        assert_eq!(prev_val, None);

        // map.insert(k1,v1); illegal.
        let k1 = String::from("k1");
        let v1 = String::from("v11");
        let prev_val : Option<String> =map.insert(k1, v1); // k1, v1 is consumed here.
        assert_eq!(prev_val, Some(String::from("v1")));

        let k1 = String::from("k1");
        let v1 = String::from("v11");
        assert_eq!(map[&k1], v1);

        // basic
        let mut map = HashMap::new();
        map.insert(1, "a".to_owned());
        map.insert(2, "b".to_owned());
        map.insert(3, "c".to_owned());
        assert_eq!(map.get(&1), Some(&"a".to_owned()));
        assert_eq!(map.get(&4), None);

        assert_eq!(map.contains_key(&1), true);
        assert_eq!(map.contains_key(&4), false);

        // insert with default
        *(map.entry(5).or_insert("d".to_owned()))= "dd".to_owned();
        assert_eq!(map.get(&5), Some(&"dd".to_owned()));

        if let Some(x) = map.get_mut(&1) {
            *x = "b".to_owned();
        }
        assert_eq!(map[&1], "b".to_owned());
        // iteration:
        // key typed as &i32, value typed as &String
        for key in map.keys() {
            // println!("{}", key);
        }

        for val in map.values_mut() {
            *val = "asd".to_owned();
        }

        for val in map.values() {
            // println!("{}", val);
        }

        for (key, val) in map.iter() {
            // println!("key: {} val: {}", key, val);
        }

        for (_, val) in map.iter_mut() {
            *val = "asd".to_owned();
        }
        assert_eq!(map.remove_entry(&1), Some((1, "asd".to_owned())));
        assert_eq!(map.remove(&1), None);
        map.clear();

    }

    pub fn orderedmap_misc() {
        use std::ops::Bound::Included;
        let mut om : BTreeMap<String, String> = BTreeMap::new();
        om.insert("k1".to_owned(), "v1".to_owned());
        om.insert("k2".to_owned(), "v2".to_owned());

        // the smallest entry
        assert_eq!(om.iter().next(), Some((&"k1".to_owned(), &"v1".to_owned())));

        // the greatest entry
        assert_eq!(om.iter().next_back(), Some((&"k2".to_owned(), &"v2".to_owned())));

        let mut map = BTreeMap::new();
        map.insert(3, "a".to_owned());
        map.insert(5, "b".to_owned());
        map.insert(8, "c".to_owned());
        // key, value typed with &
        for (key, value) in map.range((Included(&4), Included(&8))) {
            // println!("{}: {}", key, value);
        }

        for (key, value) in map.range_mut((Included(&4), Included(&8))) {
            // *key = 1; key is always immutable. 
            *value = "asdf".to_owned();
            // println!("{}: {}", key, value);
        }

        // Fast Init
        let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let b: HashSet<i32> = [4, 2, 3, 4].iter().cloned().collect();

        // Can be seen as `a - b`.
        // x typed as &
        for x in a.difference(&b) {
            // println!("{}", x); // Print 1
        }

        let diff: HashSet<i32> = a.difference(&b).cloned().collect();
        assert_eq!(diff, [1].iter().cloned().collect());

        // Note that difference is not symmetric,
        // and `b - a` means something else:
        // no cloned to get a set of reference.
        let diff: HashSet<&i32> = b.difference(&a).collect();
        assert_eq!(diff, [4].iter().collect());

        let intersection: HashSet<i32> = a.intersection(&b).cloned().collect();
        assert_eq!(intersection, [2,3].iter().cloned().collect());

        let union: HashSet<i32> = a.union(&b).cloned().collect();
        assert_eq!(union, [1,2,3,4].iter().cloned().collect());
        assert!(!a.is_subset(&b));
        assert!(!a.is_superset(&b));

    }

    pub fn hashset_misc() {
        use std::collections::HashSet;
        // Type inference lets us omit an explicit type signature (which
        // would be `HashSet<String>` in this example).
        let mut books = HashSet::new();

        // Add some books.
        books.insert("A Dance With Dragons".to_string());
        books.insert("To Kill a Mockingbird".to_string());
        books.insert("The Odyssey".to_string());
        books.insert("The Great Gatsby".to_string());

        // Check for a specific one.
        assert!(!books.contains("The Winds of Winter"));

        // Remove a book.
        assert!(books.remove("The Odyssey"));
        assert!(!books.remove("The Odyssey"));

        // book typed with &String
        for book in books.iter() {
            // println!("{}", book);
        }
   }

    pub fn orderedset_misc() {
        use std::collections::BTreeSet;
        use std::ops::Bound::Included;

        let mut set = BTreeSet::new();
        set.insert(3);
        set.insert(8);
        set.insert(5);
        // elem typed with &
        for elem in set.range((Included(&4), Included(&8))) {
            // println!("{}", elem);
        }
        assert_eq!(Some(&5), set.range(4..).next());
        // first and last
        assert_eq!(set.iter().next(), Some(&3));
        assert_eq!(set.iter().next_back(), Some(&8));
    }
}
struct Util(i32, i32);
impl Util {
    pub fn option() {
        let mut op : Option<i32> = None;
        op = Some(1);
        let op_ref : Option<&i32> = op.as_ref();
        let op_mref : Option<&mut i32> = op.as_mut();
        let op1 : Option<i32> = op.take(); // now op is None
    }

    pub fn input() {
        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    }

    pub fn unnamed_struct() {
        let mut u : Util = Util(1,2);
        assert_eq!(u.0, 1);
        assert_eq!(u.1, 2);
        u.0 = 3;
    }

    pub fn pair() {
        let p : (i32, i32) = (2,3);
        assert_eq!(p.0, 1);
        assert_eq!(p.1, 2);
    }
}

use crate::util::linked_list::{ListNode, to_list};
struct ListUtil{}
impl ListUtil {
    // traversal
    pub fn list_size(head: Option<Box<ListNode>>) -> usize {
        let mut node : &Option<Box<ListNode>> = &head;
        let mut l = 0usize;
        while node.is_some() {
            node = &node.as_ref().unwrap().next;
            l+=1;
        }
        l
    }

    // Manipulate by list heads. assume l1.size - l2.size <=1 
    pub fn take_turn_merge(l1: Option<Box<ListNode>>,l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match((l1,l2)) {
            (Some(mut l1_node), Some(mut l2_node))=>{
                l1_node.next = Self::take_turn_merge(Some(l2_node), l1_node.next);
                Some(l1_node)
            },
            (Some(l1_node), None)=>{ Some(l1_node) }, 
            (None, None)=>{ None}
            (None, Some(l2_node))=>{panic!("Impossible...")}
        }
    }

    // Manipulate by list heads.
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut reversed = None;
        let mut unreversed = head;
        while let Some(mut unreversed_node) = unreversed {
            unreversed = unreversed_node.next;
            unreversed_node.next = reversed;
            reversed = Some(unreversed_node);
        }

        reversed
    }

    // Manipulate by mutable references
    pub fn remove_1st(node: &mut Option<Box<ListNode>>) {
        //Since mutable references can not move a variable completely or partially, we have to use take() to retrieve the next node, while leaving Null the original next field. 
        node.as_mut().unwrap().next = node.as_mut().unwrap().next.as_mut().unwrap().next.take();
    }
}

struct BacktrackUtil{} 
impl BacktrackUtil {
    pub fn permute(mut elements : Vec<i32>, no_dup : bool ) -> Vec<Vec<i32>> {
        let mut result : Vec<Vec<i32>> = vec![];
        let n = elements.len();
        if n == 1 {
            result.push(vec![elements[0]]);
        }
        let mut processed : HashSet<i32> = HashSet::new();
        for i in 0..n {
            if processed.contains(&elements[i]) {
                continue;
            }
            processed.insert(elements[i]);
            elements.swap(0, i);
            let sub_elements : Vec<i32> = elements.iter().cloned().skip(1).take(n-1).collect();
            let prev_perms : Vec<Vec<i32>> = Self::permute(sub_elements, no_dup);
            for prev_perm in prev_perms {
                let mut my_perm : Vec<i32> = vec![elements[0]];
                my_perm.extend(prev_perm.clone());
                result.push(my_perm);
            }

            elements.swap(i, 0);
        }
        // println!("elements={:?}, result={:?}", elements,result);
        result
    }

    pub fn backtrack_helper<P,E,R,B>(result : &mut Vec<Vec<R>>, tmp : &mut Vec<R>, elements : &Vec<E>, predicate: P, parse: B, start : usize, no_dup : bool, element_reusable : bool) where P:Fn(&Vec<Vec<R>>, &Vec<R>)->(bool, bool) + Copy, B:Fn(&Vec<E>,usize,usize)->Option<R> + Copy, R:Clone +Eq + std::fmt::Debug, E:std::fmt::Debug{
        // is_sorted() is only supported in nightly-built rust
        // if no_dup && !elements.is_sorted() {
        //     panic!("Elements must be presorted to deduplicate.");
        // }
        if no_dup && element_reusable {
            panic!("element_reusable and no_dup can NOT be both on. ");
        }
        let (valid , early_stop) = predicate(result, tmp);
        if valid { result.push(tmp.clone()); }
        if early_stop {return}

        let n : usize = elements.len();
        let mut last_parsed : Option<R> = None;
        let mut start_parse_idx : usize = start;
        for i in start..n {
            let parsed : Option<R> = parse(elements, start, i);
            // println!("elements={:?}, start_idx={}, end={}, parsed={:?}", elements, start, i, parsed);
            if parsed.is_none() {
                continue;
            }
            let parsed : R = parsed.unwrap();

            let mut backtrack = true;
            if no_dup && last_parsed.is_some() && last_parsed.as_ref().unwrap().eq(&parsed) { 
                backtrack = false;
            }

            if backtrack {
                tmp.push(parsed.clone());
                let next_start = if element_reusable { start_parse_idx} else { i+1 };
                Self::backtrack_helper(result, tmp, elements, predicate, parse, next_start, no_dup, element_reusable);
                tmp.pop();

            }
            last_parsed = Some(parsed.clone());
            start_parse_idx = i + 1;
        }
    }

    pub fn combine(mut elements : Vec<i32>, k : usize, no_dup : bool) -> Vec<Vec<i32>>{
        let mut result : Vec<Vec<i32>> = vec![];
        let mut tmp : Vec<i32> = vec![];
        let element_reusable = false;
        if no_dup {elements.sort()}

        let predicate = |result: &Vec<Vec<i32>>, tmp : &Vec<i32>|{
            let mut valid = false;
            let mut early_stop = false;
            if tmp.len() > k {
                early_stop = true;
            } else if tmp.len() == k {
                valid = true;
            }
            (valid, early_stop)
        };

        // start_idx and end_idx are both inclusive
        let parse = |elements : &Vec<i32>, start_idx : usize, end_idx : usize|{
            Some(elements[end_idx])
        };

        Self::backtrack_helper(&mut result, &mut tmp, &elements, predicate, parse, 0, no_dup, element_reusable);
        result
    }

    pub fn subsets(mut nums: Vec<i32>, no_dup : bool) -> Vec<Vec<i32>> {
        let mut result : Vec<Vec<i32>> = vec![];
        let mut tmp : Vec<i32> = vec![];
        let element_reusable = false;

        let predicate = |result: &Vec<Vec<i32>>, tmp : &Vec<i32>|{
            let valid = true;
            let early_stop = false;
            (valid, early_stop) };
        let parse = |elements : &Vec<i32>, start_idx : usize, end_idx : usize|{
            Some(elements[end_idx])
        };
        
        Self::backtrack_helper(&mut result, &mut tmp, &nums, predicate, parse, 0, no_dup, element_reusable);
        result
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32, element_reusable : bool) -> Vec<Vec<i32>> {
        let mut result : Vec<Vec<i32>> = vec![];
        let mut tmp : Vec<i32> = vec![];
        let no_dup = false;

        let predicate = |result: &Vec<Vec<i32>>, tmp : &Vec<i32>|{
            let mut valid = false;
            let mut early_stop = false;
            let sum : i32 = tmp.iter().sum();
            if sum > target {
                early_stop = true;
            } 
            
            if sum == target {
                valid = true;
                early_stop = true;
            }
            (valid, early_stop)
        };
        let parse = |elements : &Vec<i32>, start_idx : usize, end_idx : usize|{
            Some(elements[end_idx])
        };

        Self::backtrack_helper(&mut result, &mut tmp, &candidates, predicate, parse, 0, no_dup, element_reusable);
        result
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result : Vec<Vec<String>> = vec![];
        let mut tmp : Vec<String> = vec![];
        let element_reusable = false;
        let no_dup = false;

        let predicate = |result: &Vec<Vec<String>>, tmp : &Vec<String>|{
            let early_stop = false;
            let mut l = 0usize;
            for t in tmp.iter() {
                l+=t.len();
            }
            (l==s.len(), early_stop)
        };

        let parse = |elements : &Vec<char>, start_idx : usize, end_idx : usize|{
            // end_idx is inclusive
            let chars : Vec<char> = elements.iter().skip(start_idx).take(end_idx + 1 - start_idx).cloned().collect();
            let n = chars.len();
            for i in 0..n/2 {
                if chars[i] != chars[n-1-i] {return None}
            }
            let parsed : String = chars.into_iter().collect();
            Some(parsed)
        };

        let elements : Vec<char> = s.chars().collect();
        Self::backtrack_helper(&mut result, &mut tmp, &elements, predicate, parse, 0, no_dup, element_reusable);
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

    pub fn track(board: &Vec<Vec<char>>, i : i32, j : i32, target : &String, visited: &mut HashSet<(i32,i32)>) -> bool{
        if target.len() == 0 {return true;}
        let target_char : char = target.chars().nth(0).unwrap();
        let row_count : i32 = board.len() as i32;
        let col_count : i32 = board[0].len() as i32;
        let mut found = false;
        if (!visited.contains(&(i,j)) && 0 <= i  && i < row_count && 0 <= j && j < col_count && board[i as usize][j as usize] == target_char) {
            let next_target : String = String::from(&target[1..]);
            visited.insert((i,j));
            found = Self::track(board, i-1, j, &next_target, visited) ||
            Self::track(board, i+1, j, &next_target, visited) ||
            Self::track(board, i, j-1, &next_target, visited) ||
            Self::track(board, i, j+1, &next_target, visited);
            visited.remove(&(i,j));
        }

        found
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_permcombsubset() {
        assert_eq!(BacktrackUtil::permute(vec![0,1], false), vec![vec![0,1],vec![1,0]]);

        assert_eq!(BacktrackUtil::permute(vec![1,1,2], true), vec![vec![1,1, 2],vec![1,2,1],vec![2,1,1]]);

        assert_eq!(BacktrackUtil::combine(vec![1,2,2,4], 2, false), vec![vec![1,2],vec![1,2],vec![1,4],vec![2,2],vec![2,4],vec![2,4]]);

        assert_eq!(BacktrackUtil::combine(vec![1,2,2,4], 2, true), vec![vec![1,2],vec![1,4],vec![2,2],vec![2,4]]);

        assert_eq!(
            BacktrackUtil::subsets(vec![1, 2, 2], false),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![1, 2],
                vec![2],
                vec![2, 2],
                vec![2],
            ]
        );

        assert_eq!(
            BacktrackUtil::subsets(vec![1, 2, 2], true),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2],
            ]
        );

        assert_eq!(
            BacktrackUtil::combination_sum(vec![1, 1, 1, 1, 1, 1, 1], 7,false),
            vec![vec![1, 1, 1, 1, 1, 1, 1]]
        );

        assert_eq!(
            BacktrackUtil::combination_sum(vec![1], 7,true),
            vec![vec![1, 1, 1, 1, 1, 1, 1]]
        );

        assert_eq!(
            BacktrackUtil::partition("aab".to_owned()),
            vec![ vec_string!["a", "a", "b"],vec_string!["aa", "b"],]
        );
        assert_eq!(
            BacktrackUtil::partition("aaa".to_owned()),
            vec![
                vec_string!["a", "a", "a"],
                vec_string!["a", "aa"],
                vec_string!["aa", "a"],
                vec_string!["aaa"],
            ]
        );
    }

    #[test]
    fn test_tree() {
        assert_eq!(TreeUtil::bfs(tree![1, 2, 2, 3, 4, 4, 3]), vec![1, 2, 2, 3, 4, 4, 3]);

        assert_eq!(TreeUtil::bfs(tree![1, 2, 2, null, 3, null, 3]), vec![1, 2, 2, 3, 3]);

        assert_eq!(TreeUtil::preorder(&tree![1, 2, 3, 4, null, null, 6]), vec![1, 2, 4, 3, 6]);
        assert_eq!(TreeUtil::inorder(&tree![1, 2, 3, 4, null, null, 6]), vec![4, 2, 1, 3, 6]);
        assert_eq!(TreeUtil::postorder(&tree![1, 2, 3, 4, null, null, 6]), vec![6, 3, 1, 2, 4]);
    }

    #[test]
    fn test_list() {
        assert_eq!(ListUtil::list_size(linked![1, 2, 3, 4, 5]), 5);
        assert_eq!(ListUtil::take_turn_merge(linked![1, 3, 5], linked![2, 4]), linked![1, 2, 3, 4, 5]);
        assert_eq!(ListUtil::take_turn_merge(linked![1, 3, 5], linked![2, 4, 6]), linked![1, 2, 3, 4, 5, 6]);
        assert_eq!( ListUtil::reverse_list(linked![1, 2, 3, 4, 5]), linked![5, 4, 3, 2, 1]);
        let mut l = linked![1, 2, 3, 4, 5];
        ListUtil::remove_1st(&mut l);
        assert_eq!(l, linked![1,3,4,5]);
    }

    #[test]
    fn test_bitop() {

        assert_eq!(BitOp::largest_power(9), 8);
        assert_eq!(BitOp::zero_right_digits(7, 1), 6);
        assert_eq!(BitOp::zero_right_digits(8, 2), 8);
        assert_eq!(BitOp::zero_right_digits(-3, 2), -4);

        assert_eq!(BitOp::nth_digit(-3, 0), 1);
        assert_eq!(BitOp::nth_digit(-3, 1), 0);

        assert_eq!(BitOp::nth_digit_power(-3, 1), 0);
        assert_eq!(BitOp::nth_digit_power(5, 2), 4);

        assert_eq!(BitOp::set_nth_digit_zero(5, 2), 1);
        assert_eq!(BitOp::set_nth_digit_one(5, 1), 7);

        assert_eq!(BitOp::set_zero_from_nth(5, 1), 1);
        assert_eq!(BitOp::set_zero_from_nth(5, 3), 5);
        assert_eq!(BitOp::set_zero_from_nth(-3, 3), 5);
        assert_eq!(BitOp::set_zero_from_nth(-3, 4), 13);

        assert_eq!(BitOp::set_zero_to_nth(5, 0), 4);
        assert_eq!(BitOp::set_zero_to_nth(5, 3), 0);

        assert_eq!(BitOp::set_zero_to_nth(-3, 2), -8);
        assert_eq!(BitOp::set_lsb1_zero(5), 4);
        assert_eq!(BitOp::set_lsb1_zero(6), 4);

        assert_eq!(BitOp::lsb1_power(6), 2);
        assert_eq!(BitOp::lsb1_power(7), 1);
        assert_eq!(BitOp::lsb1_power(8), 8);
    }
    #[test]
    fn test_heap() {

        let mut r = vec![12,11,13,5,6,7];
        heap_sort(&mut r);
        assert_eq!(r, vec![5,6,7,11,12,13]);

        let k1 = String::from("k1");
        let k2 = String::from("k2");
        let k3 = String::from("k3");
        let k4 = String::from("k4");

        let mut vh = VecHeap::new(vec
            ![k1.clone(),k2.clone(),k3.clone(),k4.clone()], 
            vec![1,2,3,4], 
            vec![k1.clone(),k2.clone(),k3.clone(),k4.clone()]);
        

        assert_eq!(vh.max(), Some(&(k4.clone(), 4, k4.clone())));
        assert_eq!(vh.len(), 4);

        assert_eq!(vh.remove(&k3), Some((k3.clone(), 3, k3.clone())));
        assert_eq!(vh.len(), 3);
        assert_eq!(vh.max(), Some(&(k4.clone(), 4, k4.clone())));

        assert_eq!(vh.remove(&k4), Some((k4.clone(), 4, k4.clone())));
        assert_eq!(vh.len(), 2);
        assert_eq!(vh.max(), Some(&(k2.clone(), 2, k2.clone())));

        let k5 = String::from("k5");
        vh.insert(k5.clone(), 5, k5.clone());
        assert_eq!(vh.max(), Some(&(k5.clone(), 5, k5.clone())));
        assert_eq!(vh.len(), 3);

        assert!(vh.reweight(&k1, &10));
        assert_eq!(vh.max(), Some(&(k1.clone(), 10, k1.clone())));
        assert_eq!(vh.len(), 3);

        assert_eq!(vh.remove(&k5), Some((k5.clone(), 5, k5.clone())));
        assert_eq!(vh.remove(&k4), None);
        assert_eq!(vh.remove(&k3), None);
        assert_eq!(vh.remove(&k2), Some((k2.clone(), 2, k2.clone())));
        assert_eq!(vh.remove(&k1), Some((k1.clone(), 10, k1.clone())));
        assert_eq!(vh.len(), 0);

        assert!(!vh.reweight(&k1, &10));
        assert!(!vh.reweight_with_default(&k5, &5, k5.clone()));
        assert_eq!(vh.max(), Some(&(k5.clone(), 5, k5.clone())));
        assert_eq!(vh.len(), 1);
    }

    #[test]
    fn test_binarysearch() {
        assert_eq!(BinarySearch::first_equal(vec![1,1,3,3,5,5],0), -1);
        assert_eq!(BinarySearch::first_equal(vec![1,1,3,3,5,5],1), 0);
        assert_eq!(BinarySearch::first_equal(vec![1,1,3,3,5,5],2), -1);
        assert_eq!(BinarySearch::first_equal(vec![1,1,3,3,5,5],3), 2);
        assert_eq!(BinarySearch::first_equal(vec![1,1,3,3,5,5],5), 4);
        assert_eq!(BinarySearch::first_equal(vec![1,1,3,3,5,5],7), -1);

        assert_eq!(BinarySearch::last_equal(vec![1,1,3,3,5,5],0), -1);
        assert_eq!(BinarySearch::last_equal(vec![1,1,3,3,5,5],1), 1);
        assert_eq!(BinarySearch::last_equal(vec![1,1,3,3,5,5],2), -1);
        assert_eq!(BinarySearch::last_equal(vec![1,1,3,3,5,5],3), 3);
        assert_eq!(BinarySearch::last_equal(vec![1,1,3,3,5,5],5), 5);
        assert_eq!(BinarySearch::last_equal(vec![1,1,3,3,5,5],7), -1);

        assert_eq!(BinarySearch::first_gt(vec![1,1,3,3,5,5],2), 2);
        assert_eq!(BinarySearch::first_gt(vec![1,1,3,3,5,5],3), 4);
        assert_eq!(BinarySearch::first_gt(vec![1,1,3,3,5,5],5), -1);
        assert_eq!(BinarySearch::first_gt(vec![1,1,3,3,5,5],7), -1);

        assert_eq!(BinarySearch::first_ge(vec![1,1,3,3,5,5],0), 0);
        assert_eq!(BinarySearch::first_ge(vec![1,1,3,3,5,5],1), 0);
        assert_eq!(BinarySearch::first_ge(vec![1,1,3,3,5,5],2), 2);
        assert_eq!(BinarySearch::first_ge(vec![1,1,3,3,5,5],3), 2);
        assert_eq!(BinarySearch::first_ge(vec![1,1,3,3,5,5],5), 4);
        assert_eq!(BinarySearch::first_ge(vec![1,1,3,3,5,5],7), -1);

        assert_eq!(BinarySearch::last_lt(vec![1,1,3,3,5,5],0), -1);
        assert_eq!(BinarySearch::last_lt(vec![1,1,3,3,5,5],1), -1);
        assert_eq!(BinarySearch::last_lt(vec![1,1,3,3,5,5],2), 1);
        assert_eq!(BinarySearch::last_lt(vec![1,1,3,3,5,5],3), 1);
        assert_eq!(BinarySearch::last_lt(vec![1,1,3,3,5,5],5), 3);
        assert_eq!(BinarySearch::last_lt(vec![1,1,3,3,5,5],7), 5);

        assert_eq!(BinarySearch::last_le(vec![1,1,3,3,5,5],0), -1);
        assert_eq!(BinarySearch::last_le(vec![1,1,3,3,5,5],1), 1);
        assert_eq!(BinarySearch::last_le(vec![1,1,3,3,5,5],2), 1);
        assert_eq!(BinarySearch::last_le(vec![1,1,3,3,5,5],3), 3);
        assert_eq!(BinarySearch::last_le(vec![1,1,3,3,5,5],5), 5);
        assert_eq!(BinarySearch::last_le(vec![1,1,3,3,5,5],7), 5);
    }

    #[test]
    fn test_unionfind() {
        let mut elements = HashSet::new();
        let e0 = "0".to_owned();
        let e1 = "1".to_owned();
        let e2 = "2".to_owned();
        let e3 = "3".to_owned();
        let e4 = "4".to_owned();
        let e5 = "5".to_owned();
        elements.insert(e0.clone());
        elements.insert(e1.clone());
        elements.insert(e2.clone());
        elements.insert(e3.clone());
        elements.insert(e4.clone());
        let mut uf = UnionFind::new_with(&elements);
        assert_eq!(uf.max_subset_size, 1);
        assert_eq!(uf.subset_count, 5);
        assert_eq!(uf.parents[&e3], e3);
        assert_eq!(uf.find(&e3), Some(e3.clone()));
        assert_eq!(uf.find(&e5), None);

        // shall result like
        // 0 2 3 4
        // - 
        // 1
        assert!(uf.union(&e0, &e1));
        assert_eq!(uf.max_subset_size, 2);
        assert_eq!(uf.subset_count, 4);
        assert_eq!(uf.parents[&e0], e0);
        assert_eq!(uf.parents[&e1], e0);
        assert_eq!(uf.find(&e1), Some(e0.clone()));
        assert_eq!(uf.find(&e4), Some(e4.clone()));

        // shall result like
        // 0 3 4
        // - -
        // 1 2
        assert!(uf.union(&e3, &e2));
        assert_eq!(uf.max_subset_size, 2);
        assert_eq!(uf.subset_count, 3);
        assert_eq!(uf.parents[&e2], e3);
        assert_eq!(uf.parents[&e3], e3);
        assert_eq!(uf.find(&e2), Some(e3.clone()));
        assert_eq!(uf.find(&e4), Some(e4.clone()));

        // shall result like
        // 0   3 
        // -  - -
        // 1  2 4
        assert!(uf.union(&e4, &e2));
        assert!(!uf.union(&e1, &e0));
        assert_eq!(uf.max_subset_size, 3);
        assert_eq!(uf.subset_count, 2);
        assert_eq!(uf.parents[&e2], e3);
        assert_eq!(uf.parents[&e3], e3);
        assert_eq!(uf.parents[&e4], e3);

        // shall result like
        //   3 
        // - - -
        // 0 2 4
        // -
        // 1
        assert!(uf.union(&e1, &e4));
        assert_eq!(uf.max_subset_size, 5);
        assert_eq!(uf.subset_count, 1);
        assert_eq!(uf.parents[&e0], e3);
        assert_eq!(uf.parents[&e1], e0);
        assert_eq!(uf.parents[&e2], e3);
        assert_eq!(uf.find(&e1), Some(e3.clone()));
        assert_eq!(uf.find(&e2), Some(e3.clone()));
        assert_eq!(uf.find(&e4), Some(e3.clone()));

        assert_eq!(uf.find_along_compression(&e1), Some(e3.clone()));
        // shall result like
        //   3 
        // - - - -
        // 0 1 2 4
        assert_eq!(uf.parents[&e0], e3);
        assert_eq!(uf.parents[&e1], e3);
        assert_eq!(uf.parents[&e2], e3);

        assert!(!uf.union(&e3, &e4));

        // shall result like
        //   3 
        // - - - - -
        // 0 1 2 4 5
        assert!(uf.union(&e5, &e0)); // a new element 5
        assert_eq!(uf.subset_count, 1);
        assert_eq!(uf.max_subset_size, 6);
        assert_eq!(uf.parents[&e5], e3);
        assert_eq!(uf.find(&e5), Some(e3.clone()));



        let mut uf = UnionFindInt::new(5);
        assert_eq!(uf.max_subset_size, 1);
        assert_eq!(uf.subset_count, 5);
        assert_eq!(uf.find_root(3), 3);
        assert_eq!(uf.find_root_with_compression(0), 0);

        // shall result like
        // 0 2 3 4
        // - 
        // 1
        assert!(uf.union(0, 1));
        assert_eq!(uf.max_subset_size, 2);
        assert_eq!(uf.subset_count, 4);
        assert_eq!(uf.find_root(0), 0);
        assert_eq!(uf.find_root(1), 0);
        assert_eq!(uf.find_root(2), 2);

        // shall result like
        // 0 3 4
        // - -
        // 1 2
        assert!(uf.union(3, 2));
        assert_eq!(uf.max_subset_size, 2);
        assert_eq!(uf.subset_count, 3);
        assert_eq!(uf.find_root(3), 3);
        assert_eq!(uf.find_root(4), 4);
        assert_eq!(uf.find_root_with_compression(2), 3);

        // shall result like
        // 0   3 
        // -  - -
        // 1  2 4
        assert!(uf.union(4, 2));
        assert_eq!(uf.max_subset_size, 3);
        assert_eq!(uf.subset_count, 2);
        assert_eq!(uf.find_root(1), 0);
        assert_eq!(uf.find_root(2), 3);
        assert_eq!(uf.find_root(3), 3);
        assert_eq!(uf.find_root(4), 3);

        // shall result like
        //   3 
        // - - -
        // 0 2 4
        // -
        // 1
        assert!(uf.union(1, 4));
        assert_eq!(uf.max_subset_size, 5);
        assert_eq!(uf.subset_count, 1);
        assert_eq!(uf.parents[1], 0);
        assert_eq!(uf.find_root(1), 3);
        assert_eq!(uf.find_root(0), 3);
        assert_eq!(uf.find_root(2), 3);
        assert_eq!(uf.find_root(4), 3);

        assert_eq!(uf.find_root_with_compression(1), 3);
        // shall result like
        //   3 
        // - - - -
        // 0 1 2 4
        assert_eq!(uf.max_subset_size, 5);
        assert_eq!(uf.subset_count, 1);
        assert_eq!(uf.parents[1], 3);
        assert_eq!(uf.find_root(1), 3);
        assert_eq!(uf.find_root(0), 3);
        assert_eq!(uf.find_root(2), 3);
        assert_eq!(uf.find_root(4), 3);
    }

    #[test]
    fn test_utils() {
        MapUtil::hashmap_misc();
        MapUtil::orderedmap_misc();
        MapUtil::hashset_misc();
        MapUtil::orderedset_misc();
        VecUtil::iterator_misc();
        VecUtil::misc();
    }

}
