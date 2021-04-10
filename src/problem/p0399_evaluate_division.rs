/**
 * [399] Evaluate Division
 *
 * You are given an array of variable pairs equations and an array of real numbers values, where equations[i] = [Ai, Bi] and values[i] represent the equation Ai / Bi = values[i]. Each Ai or Bi is a string that represents a single variable.
 * You are also given some queries, where queries[j] = [Cj, Dj] represents the j^th query where you must find the answer for Cj / Dj = ?.
 * Return the answers to all queries. If a single answer cannot be determined, return -1.0.
 * Note: The input is always valid. You may assume that evaluating the queries will not result in division by zero and that there is no contradiction.
 *  
 * Example 1:
 * 
 * Input: equations = [["a","b"],["b","c"]], values = [2.0,3.0], queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
 * Output: [6.00000,0.50000,-1.00000,1.00000,-1.00000]
 * Explanation: 
 * Given: a / b = 2.0, b / c = 3.0
 * queries are: a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ?
 * return: [6.0, 0.5, -1.0, 1.0, -1.0 ]
 * 
 * Example 2:
 * 
 * Input: equations = [["a","b"],["b","c"],["bc","cd"]], values = [1.5,2.5,5.0], queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
 * Output: [3.75000,0.40000,5.00000,0.20000]
 * 
 * Example 3:
 * 
 * Input: equations = [["a","b"]], values = [0.5], queries = [["a","b"],["b","a"],["a","c"],["x","y"]]
 * Output: [0.50000,2.00000,-1.00000,-1.00000]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= equations.length <= 20
 * 	equations[i].length == 2
 * 	1 <= Ai.length, Bi.length <= 5
 * 	values.length == equations.length
 * 	0.0 < values[i] <= 20.0
 * 	1 <= queries.length <= 20
 * 	queries[i].length == 2
 * 	1 <= Cj.length, Dj.length <= 5
 * 	Ai, Bi, Cj, Dj consist of lower case English letters and digits.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/evaluate-division/
// discuss: https://leetcode.com/problems/evaluate-division/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn find_root_with_compression(parents: &mut HashMap<String, String>, factor_parents: &mut HashMap<String, f64>, target : String) -> Option<(String, f64)> {
        if !parents.contains_key(&target) {return None;}

        let mut root : String = target.clone();
        let mut root_times = 1f64;
        while root != parents[&root] {
            // println!("root={}, parent_factor={}, times={}", root, factor_parents[&root], times);
            root_times *= factor_parents[&root];
            root = parents[&root].clone();
        }

        let mut target = target.clone();
        let mut times = root_times;
        while target != parents[&target] {
            let prev = parents[&target].clone();
            let prev_factor = factor_parents[&target];

            parents.insert(target.clone(), root.clone());
            factor_parents.insert(target.clone(), times);

            times /= prev_factor;
            target = prev;
        }

        
        Some((root, root_times))
    }

    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut parents = HashMap::new();
        // the parent is x times than itself. 
        let mut factor_parents = HashMap::new();
        let mut subset_sizes = HashMap::new();

        let n = equations.len();
        for i in 0..n {
            let e1_root : String; 
            let e2_root : String;
            let e1_times : f64; 
            let e2_times : f64;

            let e1 = equations[i][0].clone();
            let e2 = equations[i][1].clone();
            if let Some((root, times)) = Self::find_root_with_compression(&mut parents, &mut factor_parents, e1.clone()) {
                e1_root = root;
                e1_times = times;
            } else {
                parents.insert(e1.clone(), e1.clone());
                factor_parents.insert(e1.clone(), 1.0);
                subset_sizes.insert(e1.clone(), 1usize);

                e1_root = e1.clone();
                e1_times = 1.0;
            }

            if let Some((root, times)) = Self::find_root_with_compression(&mut parents, &mut factor_parents, e2.clone()) {
                e2_root = root;
                e2_times = times;
            } else {
                parents.insert(e2.clone(), e2.clone());
                factor_parents.insert(e2.clone(), 1.0);
                subset_sizes.insert(e2.clone(), 1usize);
                e2_root = e2.clone();
                e2_times = 1.0;
            }

            let e1_size = *subset_sizes.get(&e1_root).unwrap();
            let e2_size = *subset_sizes.get(&e2_root).unwrap();


            if e1_size < e2_size {
                parents.insert(e1_root.clone(), e2_root.clone());
                factor_parents.insert(e1_root.clone(), e2_times / e1_times / values[i]);
                subset_sizes.insert(e2_root.clone(), e1_size + e2_size);
            } else {
                parents.insert(e2_root.clone(), e1_root.clone());
                factor_parents.insert(e2_root.clone(),  e1_times * values[i] / e2_times);
                subset_sizes.insert(e1_root.clone(), e1_size + e2_size);
            }

            // println!("e1={}, e2={}, e1_root={}, e2_root={}, e1_times={}, e2_times={}", e1, e2, e1_root, e2_root, e1_times, e2_times);
            // println!("parents={:?}, factor_parents={:?}, subset_sizes={:?}", parents, factor_parents, subset_sizes);
            // println!("");

        }

        let mut result = vec![];
        for query in &queries {
            let mut q1_root : String;
            let mut q2_root : String;
            let mut q1_times : f64;
            let mut q2_times : f64;

            if let Some((root, times)) = Self::find_root_with_compression(&mut parents, &mut factor_parents, query[0].clone()) {
                q1_root = root;
                q1_times = times;
            } else {
                result.push(-1f64);
                continue;
            }

            if let Some((root, times)) = Self::find_root_with_compression(&mut parents, &mut factor_parents, query[1].clone()) {
                q2_root = root;
                q2_times = times;
            } else {
                result.push(-1f64);
                continue;
            }
            // println!("q1={}, q2={}, q1_root={}, q2_root={}, q1_times={}, q2_times={}", query[0], query[1], q1_root, q2_root, q1_times, q2_times);
            if q1_root == q2_root {
                result.push(q2_times / q1_times); 
            } else {
                result.push(-1f64);
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
    fn test_399() {
        assert_eq!(Solution::calc_equation(
            vec![vec!["a".to_owned(),"b".to_owned()],vec!["b".to_owned(),"c".to_owned()]], 
            vec![2.0,3.0], vec![vec!["a".to_owned(),"c".to_owned()],vec!["b".to_owned(),"a".to_owned()],vec!["a".to_owned(),"e".to_owned()],vec!["a".to_owned(),"a".to_owned()],vec!["x".to_owned(),"x".to_owned()]]), vec![6.00000,0.50000,-1.00000,1.00000,-1.00000]);

        assert_eq!(Solution::calc_equation(
            vec![vec!["a".to_owned(),"b".to_owned()],vec!["b".to_owned(),"c".to_owned()],vec!["bc".to_owned(),"cd".to_owned()]], 
            vec![1.5, 2.5, 5.0], vec![vec!["a".to_owned(),"c".to_owned()],vec!["c".to_owned(),"b".to_owned()],vec!["bc".to_owned(),"cd".to_owned()],vec!["cd".to_owned(),"bc".to_owned()]]), vec![3.75000,0.40000,5.00000,0.20000]);

        assert_eq!(Solution::calc_equation(
            vec![vec!["a".to_owned(),"b".to_owned()]], 
            vec![0.5], vec![vec!["a".to_owned(),"b".to_owned()],vec!["b".to_owned(),"a".to_owned()],vec!["a".to_owned(),"c".to_owned()],vec!["x".to_owned(),"y".to_owned()]]), vec![0.50000,2.00000,-1.00000,-1.00000]);
    }
}
