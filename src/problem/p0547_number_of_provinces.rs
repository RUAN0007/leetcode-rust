/**
 * [547] Number of Provinces
 *
 * There are n cities. Some of them are connected, while some are not. If city a is connected directly with city b, and city b is connected directly with city c, then city a is connected indirectly with city c.
 * A province is a group of directly or indirectly connected cities and no other cities outside of the group.
 * You are given an n x n matrix isConnected where isConnected[i][j] = 1 if the i^th city and the j^th city are directly connected, and isConnected[i][j] = 0 otherwise.
 * Return the total number of provinces.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/24/graph1.jpg" style="width: 222px; height: 142px;" />
 * Input: isConnected = [[1,1,0],[1,1,0],[0,0,1]]
 * Output: 2
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/24/graph2.jpg" style="width: 222px; height: 142px;" />
 * Input: isConnected = [[1,0,0],[0,1,0],[0,0,1]]
 * Output: 3
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 200
 * 	n == isConnected.length
 * 	n == isConnected[i].length
 * 	isConnected[i][j] is 1 or 0.
 * 	isConnected[i][i] == 1
 * 	isConnected[i][j] == isConnected[j][i]
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-provinces/
// discuss: https://leetcode.com/problems/number-of-provinces/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashSet;
use std::{collections::HashMap, hash::Hash};

struct UnionFind<T :Eq + Hash + Copy> {
    parents: HashMap<T,T>,
    subset_sizes: HashMap<T, usize>,
    pub max_subset_size: usize,
    pub subset_count: usize,
}

impl<T : Eq + Hash + Copy> UnionFind<T> {
    fn new(elements : HashSet<T>) -> UnionFind<T> {
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
        if root1 == root2 || root1.is_none() || root2.is_none() { return false}
        let root1= root1.unwrap();
        let root2 = root2.unwrap();

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
    pub fn find_circle_num(mut is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut roots = vec![0;n];
        for i in 0..n {
            roots[i] = i;
        }
        
        loop {
            let mut connected_1 = 0usize;
            let mut connected_2 = 0usize;
            // Find first i is the root. 

            let mut found_connected = false;
            for i in 0..n {
                if roots[i] != i {continue;} // skip non-root
                for j in i+1..n {
                    if is_connected[i][j] == 1 && roots[j] != i {
                        connected_1 = i;
                        connected_2 = j;
                        found_connected = true;
                        break;
                    }
                }
                if found_connected {break;}
            }

            if !found_connected {break}
            // println!("connected_1={} connected_2={}",connected_1, connected_2);
            // merge j's connectivity into i

            for k in 0..n {
                is_connected[connected_1][k] |= is_connected[roots[connected_2]][k];
            }
            for jj in 0..n {
                if roots[jj] == roots[connected_2] {
                    roots[jj] = connected_1;
                }
            }
            // println!("\troots={:?}", roots);
            // println!("\tconnected={:?}", is_connected);

        }

        let mut root_count = 0;
        for i in 0..n {
            if roots[i] == i {root_count+=1;}
        }
        root_count

    }

    // pub fn find_circle_num_uf(is_connected: Vec<Vec<i32>>) -> i32 {

    //     let mut elements = HashSet::new();
    //     let n = is_connected.len();
    //     for i in 0..n {
    //         elements.insert(i);
    //     }
    //     let mut uf = UnionFind::new(elements);
    //     for i in 0..n {
    //         for j in 0..n {
    //             if is_connected[i][j] == 1 {
    //                 uf.union(i, j);
    //             }
    //         }
    //     }

    //     uf.subset_count as i32
    // }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_547() {
        assert_eq!(Solution::find_circle_num(vec![vec![1,1,0],vec![1,1,0],vec![0,0,1]]), 2);
        assert_eq!(Solution::find_circle_num(vec![vec![1,0,0],vec![0,1,0],vec![0,0,1]]), 3);
        assert_eq!(Solution::find_circle_num(vec![vec![1,0,0,1],vec![0,1,1,0],vec![0,1,1,1],vec![1,0,1,1]]), 1);
    }
}
