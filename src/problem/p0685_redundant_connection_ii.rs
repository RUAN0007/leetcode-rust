/**
 * [685] Redundant Connection II
 *
 * In this problem, a rooted tree is a directed graph such that, there is exactly one node (the root) for which all other nodes are descendants of this node, plus every node has exactly one parent, except for the root node which has no parents.
 * The given input is a directed graph that started as a rooted tree with n nodes (with distinct values from 1 to n), with one additional directed edge added. The added edge has two different vertices chosen from 1 to n, and was not an edge that already existed.
 * The resulting graph is given as a 2D-array of edges. Each element of edges is a pair [ui, vi] that represents a directed edge connecting nodes ui and vi, where ui is a parent of child vi.
 * Return an edge that can be removed so that the resulting graph is a rooted tree of n nodes. If there are multiple answers, return the answer that occurs last in the given 2D-array.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/20/graph1.jpg" style="width: 222px; height: 222px;" />
 * Input: edges = [[1,2],[1,3],[2,3]]
 * Output: [2,3]
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/20/graph2.jpg" style="width: 222px; height: 382px;" />
 * Input: edges = [[1,2],[2,3],[3,4],[4,1],[1,5]]
 * Output: [4,1]
 * 
 *  
 * Constraints:
 * 
 * 	n == edges.length
 * 	3 <= n <= 1000
 * 	edges[i].length == 2
 * 	1 <= ui, vi <= n
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/redundant-connection-ii/
// discuss: https://leetcode.com/problems/redundant-connection-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
        // for each edge
        //   if it leads to a node with 2 in-degree:
        //      if found a cycle:
        //         return edge1;
        //      else;
        //         mark down two edges as edge1 and edge2
        //   else
        //      union two nodes
        //      if already belong to a subset. return edge1 if non-empty. else mark found cycle. 
        // return edge2 if non-empty, return last_cyclic_edge
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut parents : Vec<usize> = (0..=n).collect();
        let mut subset_sizes : Vec<usize> = vec![1usize;n+1];
        let mut in_nodes = vec![0usize;n+1]; // 0 for NA
        let mut edge1 = vec![]; // the 1st edge into a common node. 
        let mut edge2 = vec![]; // the 2nd edge into a common node
        let mut last_cycle_edge = None;
        for edge in &edges {
            let from = edge[0] as usize;
            let to = edge[1] as usize;

            if in_nodes[to] == 0 {
                // first incoming edges from node to. 
                in_nodes[to] = from;

                // perform union
                let mut from_root = from;
                while from_root != parents[from_root] {
                    from_root = parents[from_root];
                }

                let mut to_root = to;
                while to_root != parents[to_root] {
                    to_root = parents[to_root];
                }

                if from_root == to_root {
                    if edge1.len() != 0 {
                        return edge1;
                    } else {
                        // try to find an edge leading to a node with two in-degrees later. 
                        last_cycle_edge = Some(vec![from as i32, to as i32]);
                    }
                } else if subset_sizes[from] < subset_sizes[to] {
                    parents[from_root] = to_root;
                    subset_sizes[to] += subset_sizes[from];
                } else {
                    parents[to_root] = from_root;
                    subset_sizes[from] += subset_sizes[to];
                }

            } else if last_cycle_edge.is_none() {
                edge1 = vec![in_nodes[to] as i32, to as i32];
                edge2 = vec![from as i32, to as i32];

            } else {
                edge1 = vec![in_nodes[to] as i32, to as i32];
                return edge1;
            }

        }
        if last_cycle_edge.is_none() {
            edge2
        } else {
            last_cycle_edge.unwrap()
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_685() {
        assert_eq!(Solution::find_redundant_directed_connection(vec![vec![1,2],vec![2,3],vec![3,4],vec![4,1],vec![1,5]]), vec![4,1]);

        assert_eq!(Solution::find_redundant_directed_connection(vec![vec![1,2],vec![1,3],vec![2,3]]), vec![2,3]);

        assert_eq!(Solution::find_redundant_directed_connection(vec![vec![2,1],vec![3,1],vec![4,2],vec![1,4]]), vec![2,1]);

        assert_eq!(Solution::find_redundant_directed_connection(vec![vec![3,4],vec![4,1],vec![1,2],vec![2,3],vec![5,1]]), vec![4,1]);
    }
}
