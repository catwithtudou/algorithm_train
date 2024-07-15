use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
	pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
		let mut email_to_index = HashMap::new();
		for (i, account) in accounts.iter().enumerate() {
			for email in account.iter().skip(1) {
				email_to_index.entry(email.clone()).or_insert(Vec::new()).push(i);
			}
		}

		fn dfs(i: usize, accounts: &Vec<Vec<String>>, email_to_index: &HashMap<String, Vec<usize>>, vis: &mut Vec<bool>, email_set: &mut HashSet<String>) {
			vis[i] = true;
			for email in accounts[i].iter().skip(1) {
				if email_set.contains(email) {
					continue;
				}
				email_set.insert(email.clone());
				for &j in email_to_index.get(email).unwrap() {
					if !vis[j] {
						dfs(j, accounts, email_to_index, vis, email_set);
					}
				}
			}
		}

		let mut ans = vec![];
		let mut vis = vec![false; accounts.len()];
		for (i, account) in accounts.iter().enumerate() {
			if vis[i] {
				continue;
			}
			let mut email_set = HashSet::new();
			dfs(i, &accounts, &email_to_index, &mut vis, &mut email_set);

			let mut res = email_set.into_iter().collect::<Vec<_>>();
			res.sort_unstable();
			res.insert(0, account[0].clone());
			ans.push(res);
		}
		ans
	}
}