/**
 * [295] Find Median from Data Stream
 *
 * The median is the middle value in an ordered integer list. If the size of the list is even, there is no middle value and the median is the mean of the two middle values.
 * 
 * 	For example, for arr = [2,3,4], the median is 3.
 * 	For example, for arr = [2,3], the median is (2 + 3) / 2 = 2.5.
 * 
 * Implement the MedianFinder class:
 * 
 * 	MedianFinder() initializes the MedianFinder object.
 * 	void addNum(int num) adds the integer num from the data stream to the data structure.
 * 	double findMedian() returns the median of all elements so far. Answers within 10^-5 of the actual answer will be accepted.
 * 
 *  
 * Example 1:
 * 
 * Input
 * ["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
 * [[], [1], [2], [], [3], []]
 * Output
 * [null, null, null, 1.5, null, 2.0]
 * Explanation
 * MedianFinder medianFinder = new MedianFinder();
 * medianFinder.addNum(1);    // arr = [1]
 * medianFinder.addNum(2);    // arr = [1, 2]
 * medianFinder.findMedian(); // return 1.5 (i.e., (1 + 2) / 2)
 * medianFinder.addNum(3);    // arr[1, 2, 3]
 * medianFinder.findMedian(); // return 2.0
 * 
 *  
 * Constraints:
 * 
 * 	-10^5 <= num <= 10^5
 * 	There will be at least one element in the data structure before calling findMedian.
 * 	At most 5 * 10^4 calls will be made to addNum and findMedian.
 * 
 *  
 * Follow up:
 * 
 * 	If all integer numbers from the stream are in the range [0, 100], how would you optimize your solution?
 * 	If 99% of all integer numbers from the stream are in the range [0, 100], how would you optimize your solution?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-median-from-data-stream/
// discuss: https://leetcode.com/problems/find-median-from-data-stream/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use super::p0000_template::VecHeap;

// use  std::cmp::Ordering;
// use std::{collections::HashMap, hash::Hash};
// #[derive(Debug)]
// struct VecHeap<K: Clone + Hash + Eq, W:Ord + Clone, V: Clone>{
//     elements: Vec<(K,W,V)>,
//     key2idx: HashMap<K, usize>,
// } 

// impl<K: Clone + Hash + Eq, W:Ord + Clone, V: Clone> VecHeap<K,W,V>{
//     pub fn new(keys: Vec<K>, weights : Vec<W>, values : Vec<V>) -> VecHeap<K,W,V> {
//         let mut vh = VecHeap{elements: vec![], key2idx: HashMap::new()};
//         let n = keys.len();
//         for i in 0..keys.len() {
//             let idx = vh.elements.len();
//             vh.key2idx.insert(keys[i].clone(), idx);
//             vh.elements.push((keys[i].clone(), weights[i].clone(), values[i].clone()));
//         }

//         for i in (0..(n/2)).rev() {
//             vh.topdown_heapify(i, n);
//         }

//         vh
//     }

//     pub fn reweight_with_default(&mut self, key: &K, weight: &W, default_value: V) -> bool {
//         if let Some((k,w,v)) = self.remove(key) {
//             self.insert(k, weight.clone(),v);
//             true
//         } else {
//             self.insert(key.clone(), weight.clone(),default_value);
//             false
//         }
//     }

//     pub fn reweight(&mut self, key: &K, weight: &W) -> bool {
//         if let Some((k,w,v)) = self.remove(key) {
//             self.insert(k, weight.clone(),v);
//             true
//         } else {
//             false
//         }
//     }

//     pub fn remove(&mut self, key : &K) -> Option<(K,W,V)> {
//         if let Some(&removed_pos) = self.key2idx.get(key) {
//             // swap the removed element with the last element. 
//             self.key2idx.remove(key);
//             if removed_pos == self.elements.len() - 1 {
//                 self.elements.pop()
//             } else  {
//                 //swap the removed element wit the last. 
//                 let removed = self.elements[removed_pos].clone();
//                 let last_entry = self.elements.pop().unwrap();
//                 self.key2idx.insert(last_entry.0.clone(), removed_pos);
//                 self.elements[removed_pos] = last_entry;

//                 // topdown_heapify from the removed pos
//                 self.topdown_heapify(removed_pos, self.len());
//                 Some(removed)
//             }

//         } else {
//             None
//         }
//     }

//     pub fn insert(&mut self, key: K, weight: W, value: V) {
//         let last_pos = self.elements.len();
//         self.key2idx.insert(key.clone(), last_pos);
//         self.elements.push((key, weight, value));
//         self.bottomup_heapify(last_pos)
//     }

//     pub fn bottomup_heapify(&mut self, start_pos : usize) {
//         if 0 < start_pos  {
//             let parent_pos = (start_pos + 1) / 2 - 1;
//             if self.elements[parent_pos].1.cmp(&self.elements[start_pos].1) == Ordering::Less {
//                 self.elements.swap(parent_pos, start_pos);
//                 self.key2idx.insert(self.elements[start_pos].0.clone(), start_pos);
//                 self.key2idx.insert(self.elements[parent_pos].0.clone(), parent_pos);
//                 self.bottomup_heapify(parent_pos);
//             }
//         }
//     }
    
//     pub fn topdown_heapify(&mut self, start_pos: usize, max_len: usize ) {
//         let left_pos = 2 * start_pos + 1;
//         let right_pos = 2 * (start_pos + 1);

//         let mut large_pos = None;
//         let mut large_weight = self.elements[start_pos].1.clone();
//         if left_pos < max_len && large_weight.cmp(&self.elements[left_pos].1) == Ordering::Less {
//             large_weight = self.elements[left_pos].1.clone();
//             large_pos = Some(left_pos);
//         }

//         if right_pos < max_len && large_weight.cmp(&self.elements[right_pos].1) == Ordering::Less {
//             large_weight = self.elements[right_pos].1.clone();
//             large_pos = Some(right_pos);
//         }

//         if let Some(large_pos) = large_pos {
//             self.elements.swap(start_pos, large_pos);
//             self.key2idx.insert(self.elements[start_pos].0.clone(), start_pos);
//             self.key2idx.insert(self.elements[large_pos].0.clone(), large_pos);

//             self.topdown_heapify(large_pos, max_len);
//         }
//     }

//     pub fn max(&self) -> Option<&(K,W,V)> {
//         self.elements.get(0)
//     }

//     pub fn len(&self) -> usize {
//         self.elements.len()
//     }

// }

struct MedianFinder {
        first_half_max_heap : VecHeap<usize, i32, i32>,
        sec_half_min_heap : VecHeap<usize, i32, i32>
    }


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    /** initialize your data structure here. */
    fn new() -> Self {
        MedianFinder{
            first_half_max_heap : VecHeap::new(vec![], vec![], vec![]),
            sec_half_min_heap : VecHeap::new(vec![], vec![], vec![])
        } 
    }
    
    fn add_num(&mut self, num: i32) {
        let first_half_len : usize = self.first_half_max_heap.len();
        let sec_half_len : usize = self.sec_half_min_heap.len();
        let num_id : usize = first_half_len + sec_half_len; // make each key distinct

        if  first_half_len == sec_half_len {
            self.first_half_max_heap.insert(num_id, num, num);
            let &(first_half_greatest_id, _, first_half_greatest) = self.first_half_max_heap.max().unwrap();
            self.first_half_max_heap.remove(&first_half_greatest_id);
            // println!("\tEVEN first_half_greatest: {}", first_half_greatest);
            self.sec_half_min_heap.insert(first_half_greatest_id, -first_half_greatest, first_half_greatest);
        } else {
            self.sec_half_min_heap.insert(num_id, -num, num);
            let &(sec_half_smallest_id, _, sec_half_smallest) = self.sec_half_min_heap.max().unwrap();

            // println!("\tODD sec_half_smallest: {}", sec_half_smallest);
            self.sec_half_min_heap.remove(&sec_half_smallest_id);
            self.first_half_max_heap.insert(sec_half_smallest_id, sec_half_smallest, sec_half_smallest);
        }

        // println!("\tFirst Half Max Heap LEN({}): {:?}", self.first_half_max_heap.len(), self.first_half_max_heap.elements);
        // println!("\tSec Half Min Heap LEN({}): {:?}", self.sec_half_min_heap.len(), self.sec_half_min_heap.elements);
    }
    
    fn find_median(&self) -> f64 {
        let &(_, _, larger_mid) = self.sec_half_min_heap.max().unwrap();
        let &(_, _, smaller_mid) = self.sec_half_min_heap.max().unwrap();

        if self.sec_half_min_heap.len() == self.first_half_max_heap.len() {
            let &(_, _, smaller_mid) = self.first_half_max_heap.max().unwrap();
            // println!("\tEVEN smaller_mid = {}", smaller_mid);
            // println!("\tEVEN larger_mid = {}", larger_mid);
            (smaller_mid as f64 + larger_mid as f64) / 2.0
        } else {
            // println!("\tODD smaller_mid = {}", smaller_mid);
            // println!("\tODD larger_mid = {}", larger_mid);
            (smaller_mid as f64 + larger_mid as f64) / 2.0
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn test_295() {

        fn helper(label : String, cmds : Vec<String>, parameters : Vec<i32>, results : Vec<f64>) {
            let mut mf = MedianFinder::new();
            for (i, cmd) in cmds.iter().enumerate() {
                if cmd.eq("addNum") {
                    // println!("{} Add Num {}", i, parameters[i]);
                    mf.add_num(parameters[i]);
                } else if cmd.eq("findMedian") {
                    // println!("{} FindMedian", i);
                    let actual : f64 = mf.find_median();
                    let expected : f64 = results[i];
                    if actual != results[i] {
                        panic!("label={}, i={}, mf.findMedian={}, result={}", label, i, actual, expected);
                    }
                }
            }
        }

        helper("test1".to_owned(), vec!["addNum".to_owned(), "addNum".to_owned(), "findMedian".to_owned(), "addNum".to_owned(), "findMedian".to_owned()], vec![1,2,0,3,0], 
        vec![0.0,0.0,1.5, 0.0, 2.0]);

        let ops : Vec<String> = ["MedianFinder","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian"].iter().map(|x|{String::from(*x)}).collect();
        let parameters : Vec<i32> = vec![vec![],vec![-1],vec![],vec![-2],vec![],vec![-3],vec![],vec![-4],vec![],vec![-5],vec![]].iter().map(|x|{if x.len() == 0 {0}else{x[0]}}).collect();
        let expected : Vec<f64> = vec![0.0,0.0,-1.00000,0.0,-1.50000,0.0,-2.00000,0.0,-2.50000,0.0,-3.00000];

        helper("test2".to_owned(), ops, parameters, expected);

        let ops : Vec<String> = ["MedianFinder","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian","addNum","findMedian"].iter().map(|x|{String::from(*x)}).collect();
        let parameters : Vec<i32> = vec![vec![],vec![78],vec![],vec![14],vec![],vec![50],vec![],vec![20],vec![],vec![13],vec![],vec![9],vec![],vec![25],vec![],vec![8],vec![],vec![13],vec![],vec![37],vec![],vec![29],vec![],vec![33],vec![],vec![55],vec![],vec![52],vec![],vec![6],vec![],vec![17],vec![],vec![65],vec![],vec![23],vec![],vec![74],vec![],vec![43],vec![],vec![5],vec![],vec![29],vec![],vec![29],vec![],vec![72],vec![],vec![7],vec![],vec![13],vec![],vec![56],vec![],vec![21],vec![],vec![31],vec![],vec![66],vec![],vec![69],vec![],vec![69],vec![],vec![74],vec![],vec![12],vec![],vec![77],vec![],vec![23],vec![],vec![10],vec![],vec![6],vec![],vec![27],vec![],vec![63],vec![],vec![77],vec![],vec![21],vec![],vec![40],vec![],vec![10],vec![],vec![19],vec![],vec![59],vec![],vec![35],vec![],vec![40],vec![],vec![44],vec![],vec![4],vec![],vec![15],vec![],vec![29],vec![],vec![63],vec![],vec![27],vec![],vec![46],vec![],vec![56],vec![],vec![0],vec![],vec![60],vec![],vec![72],vec![],vec![35],vec![],vec![54],vec![],vec![50],vec![],vec![14],vec![],vec![29],vec![],vec![62],vec![],vec![24],vec![],vec![18],vec![],vec![79],vec![],vec![16],vec![],vec![19],vec![],vec![8],vec![],vec![77],vec![],vec![10],vec![],vec![21],vec![],vec![66],vec![],vec![42],vec![],vec![76],vec![],vec![14],vec![],vec![58],vec![],vec![20],vec![],vec![0],vec![]].iter().map(|x|{if x.len() == 0 {0}else{x[0]}}).collect();

        let expected : Vec<f64> = vec![0.0,0.0,78.00000,0.0,46.00000,0.0,50.00000,0.0,35.00000,0.0,20.00000,0.0,17.00000,0.0,20.00000,0.0,17.00000,0.0,14.00000,0.0,17.00000,0.0,20.00000,0.0,22.50000,0.0,25.00000,0.0,27.00000,0.0,25.00000,0.0,22.50000,0.0,25.00000,0.0,24.00000,0.0,25.00000,0.0,27.00000,0.0,25.00000,0.0,27.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,27.00000,0.0,29.00000,0.0,27.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,30.00000,0.0,31.00000,0.0,32.00000,0.0,31.00000,0.0,30.00000,0.0,31.00000,0.0,30.00000,0.0,29.00000,0.0,30.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000,0.0,29.00000];

        helper("test3".to_owned(), ops, parameters, expected);

    }
}
