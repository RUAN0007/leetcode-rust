/**
 * [684] Redundant Connection
 *
 * 
 * In this problem, a tree is an undirected graph that is connected and has no cycles.
 * 
 * The given input is a graph that started as a tree with N nodes (with distinct values 1, 2, ..., N), with one additional edge added.  The added edge has two different vertices chosen from 1 to N, and was not an edge that already existed.
 * 
 * The resulting graph is given as a 2D-array of edges.  Each element of edges is a pair [u, v] with u < v, that represents an undirected edge connecting nodes u and v.
 * 
 * Return an edge that can be removed so that the resulting graph is a tree of N nodes.  If there are multiple answers, return the answer that occurs last in the given 2D-array.  The answer edge [u, v] should be in the same format, with u < v.
 * Example 1:<br />
 * 
 * Input: [[1,2], [1,3], [2,3]]
 * Output: [2,3]
 * Explanation: The given undirected graph will be like this:
 *   1
 *  / \
 * 2 - 3
 * 
 * 
 * Example 2:<br />
 * 
 * Input: [[1,2], [2,3], [3,4], [1,4], [1,5]]
 * Output: [1,4]
 * Explanation: The given undirected graph will be like this:
 * 5 - 1 - 2
 *     |   |
 *     4 - 3
 * 
 * 
 * Note:<br />
 * The size of the input 2D-array will be between 3 and 1000.
 * Every integer represented in the 2D-array will be between 1 and N, where N is the size of the input array.
 * 
 * 
 * <br />
 * 
 * 
 * <font color="red">Update (2017-09-26):</font><br>
 * We have overhauled the problem description + test cases and specified clearly the graph is an undirected graph. For the directed graph follow up please see <a href="https://leetcode.com/problems/redundant-connection-ii/description/">Redundant Connection II</a>). We apologize for any inconvenience caused.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/redundant-connection/
// discuss: https://leetcode.com/problems/redundant-connection/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_root_with_compress(parents: &mut Vec<usize>, target : usize) -> usize {
        let mut root = target;
        while root != parents[root] {
            root = parents[root];
        }

        let mut target = target;
        while target != root {
            let tmp = parents[target];
            parents[target] = root;
            target = tmp;
        }
        root
    }

    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut parents : Vec<usize> = (0..=n).collect();
        let mut subset_sizes = vec![1;n+1];

        for edge in &edges {
            let e1 = Self::find_root_with_compress(&mut parents, edge[0] as usize);
            let e2 = Self::find_root_with_compress(&mut parents, edge[1] as usize);
            if e1 == e2 {
                return edge.clone();
            } else if subset_sizes[e1] < subset_sizes[e2] {
                parents[e1] = e2;
                subset_sizes[e2] += subset_sizes[e1];
            } else {
                parents[e2] = e1;
                subset_sizes[e1] += subset_sizes[e2];
            }
        }
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_684() {
    }
}
