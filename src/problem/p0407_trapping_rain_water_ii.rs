/**
 * [407] Trapping Rain Water II
 *
 * Given an m x n integer matrix heightMap representing the height of each unit cell in a 2D elevation map, return the volume of water it can trap after raining.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/trap1-3d.jpg" style="width: 361px; height: 321px;" />
 * Input: heightMap = [[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]]
 * Output: 4
 * Explanation: After the rain, water is trapped between the blocks.
 * We have two small pounds 1 and 3 units trapped.
 * The total volume of water trapped is 4.
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/trap2-3d.jpg" style="width: 401px; height: 321px;" />
 * Input: heightMap = [[3,3,3,3,3],[3,2,2,2,3],[3,2,1,2,3],[3,2,2,2,3],[3,3,3,3,3]]
 * Output: 10
 * 
 *  
 * Constraints:
 * 
 * 	m == heightMap.length
 * 	n == heightMap[i].length
 * 	1 <= m, n <= 200
 * 	0 <= heightMap[i][j] <= 2 * 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/trapping-rain-water-ii/
// discuss: https://leetcode.com/problems/trapping-rain-water-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use super::p0000_template::VecHeap;

// use std::cmp::Ordering;
// use std::{collections::HashMap, hash::Hash};
// #[derive(Debug)]
// pub struct VecHeap<K: Clone + Hash + Eq, W:Ord + Clone, V: Clone>{
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

//     pub fn insert(&mut self, key: K, weight: W, value: V) -> bool {
//         if self.key2idx.get(&key).is_none() {
//             let last_pos = self.elements.len();
//             self.key2idx.insert(key.clone(), last_pos);
//             self.elements.push((key, weight, value));
//             self.bottomup_heapify(last_pos);
//             true
//         } else {
//             false
//         }

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

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        // the trapped water volume of a unit is determined by the lowest unit among all the max units in each path towards the boundary.

        // Since all such paths from cell[i][j] must include itself, hence lowest_heights[i][j] >= height_map[i][j]
        // -1 implies not considered yet. 
        let row_count : usize = height_map.len();
        let col_count : usize = height_map[0].len();

        let mut lowest_heights : Vec<Vec<i32>> = vec![vec![-1;col_count];row_count];

        // value is useless. 
        let mut vh : VecHeap<(usize, usize), i32, i32> = VecHeap::new(
            vec![], vec![], vec![]);

        // For all boundary cases, since it can directly reach boundary by itself, we can achieve the equality, where lowest_heights[i][j] == height_map[i][j]

        for i in 0..row_count {
            vh.insert((i, 0), -height_map[i][0], 0);
            lowest_heights[i][0] = height_map[i][0];

            vh.insert((i, col_count-1), -height_map[i][col_count-1], 0);
            lowest_heights[i][col_count-1] = height_map[i][col_count-1];
        }

        for j in 0..col_count {
            // minus to in order to get min with the max heap
            vh.insert((0, j), -height_map[0][j], 0);
            lowest_heights[0][j] = height_map[0][j];

            vh.insert((row_count - 1, j), -height_map[row_count - 1][j], 0);
            lowest_heights[row_count - 1][j] = height_map[row_count - 1][j];
        }

        let mut result : i32 = 0;
        while vh.len() != 0 {
            let &((i, j), height, _) = vh.max().unwrap();
            let height = - height;
            vh.remove(&(i,j));

            // for any neighbor of cell (i,j), classify its boundary paths into: 
            //   (1) Paths which includes cell (i,j)
            //   (2) Paths without cell (i,j)
            //  We already know the min among the max height of each path in (1) as lowest_heights[i][j], 
            //  We know each path in (2) must pass a cell in vh. Due to the priority queue nature, their height is >= lowest_heights[i].

            // if height of the neighbor < lowest_heights[i][j]:
            //   this cell can hold the water.
            //   lowest_heights[neighbor] = lowest_heights[i][j];
            // else:
            //   lowest_heights[neighbor]=height[neighbor], this optimal equal case achieves due to the presence of path (1), whose including cell heights are all <= lowest_heights[i][j]. 
            if i+1 < row_count && lowest_heights[i+1][j] == -1 {
                if height_map[i+1][j] < lowest_heights[i][j] {
                    result += lowest_heights[i][j] - height_map[i+1][j];
                    lowest_heights[i+1][j] = lowest_heights[i][j];
                } else {
                    lowest_heights[i+1][j] = height_map[i+1][j];
                }
                vh.insert((i+1,j), -lowest_heights[i+1][j], 0);
            }

            if 0 < i && lowest_heights[i-1][j] == -1 {
                if height_map[i-1][j] < lowest_heights[i][j] {
                    result += lowest_heights[i][j] - height_map[i-1][j];
                    lowest_heights[i-1][j] = lowest_heights[i][j];
                } else {
                    lowest_heights[i-1][j] = height_map[i-1][j];
                }
                vh.insert((i-1,j), -lowest_heights[i-1][j], 0);
            }

            if j+1 < col_count && lowest_heights[i][j+1] == -1 {
                if height_map[i][j+1] < lowest_heights[i][j] {
                    result += lowest_heights[i][j] - height_map[i][j+1];
                    lowest_heights[i][j+1] = lowest_heights[i][j];
                } else {
                    lowest_heights[i][j+1] = height_map[i][j+1];
                }
                vh.insert((i,j+1), -lowest_heights[i][j+1], 0);
            }

            if 0 < j && lowest_heights[i][j-1] == -1 {
                if height_map[i][j-1] < lowest_heights[i][j] {
                    result += lowest_heights[i][j] - height_map[i][j-1];
                    lowest_heights[i][j-1] = lowest_heights[i][j];
                } else {
                    lowest_heights[i][j-1] = height_map[i][j-1];
                }
                vh.insert((i,j-1), -lowest_heights[i][j-1], 0);
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_407() {
        assert_eq!(Solution::trap_rain_water(vec![vec![1,4,3,1,3,2],vec![3,2,1,3,2,4],vec![2,3,3,2,3,1]]), 4);

        assert_eq!(Solution::trap_rain_water(vec![vec![3,3,3,3,3],vec![3,2,2,2,3],vec![3,2,1,2,3],vec![3,2,2,2,3],vec![3,3,3,3,3]]), 10);

        assert_eq!(Solution::trap_rain_water(vec![vec![14,17,18,16,14,16],vec![17,3,10,2,3,8],vec![11,10,4,7,1,7],vec![13,7,2,9,8,10],vec![13,1,3,4,8,6],vec![20,3,3,9,10,8]]), 25);
    }
}
