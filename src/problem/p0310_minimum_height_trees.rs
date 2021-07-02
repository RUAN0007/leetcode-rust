/**
 * [310] Minimum Height Trees
 *
 * A tree is an undirected graph in which any two vertices are connected by exactly one path. In other words, any connected graph without simple cycles is a tree.
 * Given a tree of n nodes labelled from 0 to n - 1, and an array of n - 1 edges where edges[i] = [ai, bi] indicates that there is an undirected edge between the two nodes ai and bi in the tree, you can choose any node of the tree as the root. When you select a node x as the root, the result tree has height h. Among all possible rooted trees, those with minimum height (i.e. min(h))  are called minimum height trees (MHTs).
 * Return a list of all MHTs' root labels. You can return the answer in any order.
 * The height of a rooted tree is the number of edges on the longest downward path between the root and a leaf.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/01/e1.jpg" style="width: 800px; height: 213px;" />
 * Input: n = 4, edges = [[1,0],[1,2],[1,3]]
 * Output: [1]
 * Explanation: As shown, the height of the tree is 1 when the root is the node with label 1 which is the only MHT.
 * 
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/01/e2.jpg" style="width: 800px; height: 321px;" />
 * Input: n = 6, edges = [[3,0],[3,1],[3,2],[3,4],[5,4]]
 * Output: [3,4]
 * 
 * Example 3:
 * 
 * Input: n = 1, edges = []
 * Output: [0]
 * 
 * Example 4:
 * 
 * Input: n = 2, edges = [[0,1]]
 * Output: [0,1]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 2 * 10^4
 * 	edges.length == n - 1
 * 	0 <= ai, bi < n
 * 	ai != bi
 * 	All the pairs (ai, bi) are distinct.
 * 	The given input is guaranteed to be a tree and there will be no repeated edges.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-height-trees/
// discuss: https://leetcode.com/problems/minimum-height-trees/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn find_min_height_trees( n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {return vec![0];}
        let mut n : usize = n as usize;
        let mut adj : HashMap<i32, HashSet<i32>> = HashMap::new();

        for edge in edges.iter() {
            let n1 : i32 = edge[0];
            let n2 : i32 = edge[1];
            adj.entry(n1).or_insert(HashSet::new()).insert(n2);
            adj.entry(n2).or_insert(HashSet::new()).insert(n1);
        }
        let mut leaves : HashSet<i32> = adj.iter().filter(|&(key, val)|{val.len() == 1}).map(|(&key, _)|{key}).collect();

        while n > 2 {
            n -= leaves.len();
            let mut next_leaves : HashSet<i32> = HashSet::new();
            for &leaf in leaves.iter() {
                let parent : i32 = *adj[&leaf].iter().next().unwrap();
                adj.get_mut(&parent).unwrap().remove(&leaf);
                if adj[&parent].len() == 1 {next_leaves.insert(parent);}
            }
            leaves = next_leaves;
        }

        leaves.into_iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_310() {
        assert_eq!(
            Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]),
            vec![1]
        );
        assert_eq!(
            Solution::find_min_height_trees(
                6,
                vec![vec![0, 3], vec![1, 3], vec![2, 3], vec![4, 3], vec![5, 4]]
            ),
            vec![3, 4]
        );
        assert_eq!(Solution::find_min_height_trees(1, vec![]), vec![0]);
    }
}
