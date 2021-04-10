/**
 * [721] Accounts Merge
 *
 * Given a list of accounts where each element accounts[i] is a list of strings, where the first element accounts[i][0] is a name, and the rest of the elements are emails representing emails of the account.
 * Now, we would like to merge these accounts. Two accounts definitely belong to the same person if there is some common email to both accounts. Note that even if two accounts have the same name, they may belong to different people as people could have the same name. A person can have any number of accounts initially, but all of their accounts definitely have the same name.
 * After merging the accounts, return the accounts in the following format: the first element of each account is the name, and the rest of the elements are emails in sorted order. The accounts themselves can be returned in any order.
 *  
 * Example 1:
 * 
 * Input: accounts = [["John","johnsmith@mail.com","john_newyork@mail.com"],["John","johnsmith@mail.com","john00@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]
 * Output: [["John","john00@mail.com","john_newyork@mail.com","johnsmith@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]
 * Explanation:
 * The first and third John's are the same person as they have the common email "johnsmith@mail.com".
 * The second John and Mary are different people as none of their email addresses are used by other accounts.
 * We could return these lists in any order, for example the answer [['Mary', 'mary@mail.com'], ['John', 'johnnybravo@mail.com'], 
 * ['John', 'john00@mail.com', 'john_newyork@mail.com', 'johnsmith@mail.com']] would still be accepted.
 * 
 * Example 2:
 * 
 * Input: accounts = [["Gabe","Gabe0@m.co","Gabe3@m.co","Gabe1@m.co"],["Kevin","Kevin3@m.co","Kevin5@m.co","Kevin0@m.co"],["Ethan","Ethan5@m.co","Ethan4@m.co","Ethan0@m.co"],["Hanzo","Hanzo3@m.co","Hanzo1@m.co","Hanzo0@m.co"],["Fern","Fern5@m.co","Fern1@m.co","Fern0@m.co"]]
 * Output: [["Ethan","Ethan0@m.co","Ethan4@m.co","Ethan5@m.co"],["Gabe","Gabe0@m.co","Gabe1@m.co","Gabe3@m.co"],["Hanzo","Hanzo0@m.co","Hanzo1@m.co","Hanzo3@m.co"],["Kevin","Kevin0@m.co","Kevin3@m.co","Kevin5@m.co"],["Fern","Fern0@m.co","Fern1@m.co","Fern5@m.co"]]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= accounts.length <= 1000
 * 	2 <= accounts[i].length <= 10
 * 	1 <= accounts[i][j] <= 30
 * 	accounts[i][0] consists of English letters.
 * 	accounts[i][j] (for j > 0) is a valid email.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/accounts-merge/
// discuss: https://leetcode.com/problems/accounts-merge/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut email2name = HashMap::new();
        let mut email_parents : HashMap<String, String> = HashMap::new();
        for account in &accounts {
            let name = account[0].clone();

            // If an email already exists, find its root email 
            // If none, arbitrarly select the first email.
            let mut this_parent = account[1].clone();
            for i in 1..account.len() {
                let email = account[i].clone(); 
                if email_parents.contains_key(&email) {

                   this_parent = account[i].clone(); 
                   while this_parent != email_parents[&this_parent] {
                       this_parent = email_parents[&this_parent].clone();
                   }
                   break;
                }
            }

            // All emails update the above found email as the root. 
            for i in 1..account.len() {
                let email = account[i].clone(); 
                let mut this_root = account[i].clone(); 
                if email_parents.contains_key(&email) {
                    while this_root != email_parents[&this_root] {
                        this_root = email_parents[&this_root].clone();
                    }
                } else {
                    email2name.insert(email.clone(), name.clone());
                }

                email_parents.insert(this_root.clone(), this_parent.clone());
            }
        }

        let mut email_result = vec![];
        let mut root2result_pos = HashMap::new();
        for (email, parent) in email_parents.iter() {
            if email == parent {
                // find a root. 
                root2result_pos.insert(email.clone(), email_result.len());
                email_result.push(vec![]);
            }
        }


        for (email, parent) in email_parents.iter() {
            let mut acc = vec![email2name[email].clone()];
            let mut email_root = email.clone();
            while email_root != email_parents[&email_root] {
                email_root = email_parents[&email_root].clone();
            }
            let result_pos = root2result_pos[&email_root];
            email_result[result_pos].push(email.clone());
        }

        let mut result = vec![];
        for (root_email, &pos) in root2result_pos.iter() {
            email_result[pos].sort();
            let acc = email2name[root_email].clone();
            let mut r = vec![acc];
            r.append(&mut email_result[pos]);
            result.push(r);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_721() {
        // assert_eq!(Solution::accounts_merge(vec![vec!["John".to_owned(),"johnsmith@mail.com".to_owned(),"john_newyork@mail.com".to_owned()],vec!["John".to_owned(),"johnsmith@mail.com".to_owned(),"john00@mail.com".to_owned()],vec!["Mary".to_owned(),"mary@mail.com".to_owned()],vec!["John".to_owned(),"johnnybravo@mail.com".to_owned()]]), vec![vec!["John".to_owned(),"john00@mail.com".to_owned(),"john_newyork@mail.com".to_owned(),"johnsmith@mail.com".to_owned()],vec!["Mary".to_owned(),"mary@mail.com".to_owned()],vec!["John".to_owned(),"johnnybravo@mail.com".to_owned()]]
        // );
    }
}
