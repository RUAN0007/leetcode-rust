use std::collections::{HashMap, HashSet};

/**
 * [207] Course Schedule
 *
 * There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.
 * 
 * 	For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
 * 
 * Return true if you can finish all courses. Otherwise, return false.
 *  
 * Example 1:
 * 
 * Input: numCourses = 2, prerequisites = [[1,0]]
 * Output: true
 * Explanation: There are a total of 2 courses to take. 
 * To take course 1 you should have finished course 0. So it is possible.
 * 
 * Example 2:
 * 
 * Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
 * Output: false
 * Explanation: There are a total of 2 courses to take. 
 * To take course 1 you should have finished course 0, and to take course 0 you should also have finished course 1. So it is impossible.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= numCourses <= 10^5
 * 	0 <= prerequisites.length <= 5000
 * 	prerequisites[i].length == 2
 * 	0 <= ai, bi < numCourses
 * 	All the pairs prerequisites[i] are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/course-schedule/
// discuss: https://leetcode.com/problems/course-schedule/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
#[derive(Clone, Debug)]
struct NodeInfo{in_degree: usize, out_nodes: Vec<i32>}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph = vec![NodeInfo{in_degree: 0, out_nodes: vec![]}; num_courses as usize];
        for dep in prerequisites {
            let in_node = dep[0];
            let out_node = dep[1];
            graph[out_node as usize].in_degree += 1;
            graph[in_node as usize].out_nodes.push(out_node);
        }

        // repeatively loop graph until either happens
        //   * all node.in == -1, return true
        //   * some -1, some > 1 but no 0, return false
        let mut processed = HashSet::new();
        let mut found_root = true;
        while found_root {
            let mut decre_out_nodes = vec![];
            for (node_idx, node_info) in graph.iter_mut().enumerate() {
                let node_idx = node_idx as i32;
                if processed.contains(&node_idx) {
                    continue;
                }

                if node_info.in_degree == 0 {
                    processed.insert(node_idx as i32);
                    if 0 < node_info.out_nodes.len() {
                        decre_out_nodes = node_info.out_nodes.clone();
                        // println!("Found root for {}", node_idx);
                        break;
                    }
                }
            }

            found_root = false;
            for out_node in decre_out_nodes {
                if processed.contains(&out_node) {
                    panic!("Shall not process {} before", &out_node);
                }
                if graph[out_node as usize].in_degree <= 0 {
                    panic!("Shall not get non-positive indegree for {}", out_node);
                }
                found_root = true;
                graph[out_node as usize].in_degree -= 1;
            }
        }

        return processed.len() == num_courses as usize;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_207() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
        assert_eq!(
            Solution::can_finish(
                8,
                vec![
                    vec![1, 0],
                    vec![2, 6],
                    vec![1, 7],
                    vec![6, 4],
                    vec![7, 0],
                    vec![0, 5]
                ]
            ),
            true
        );
    }
}
