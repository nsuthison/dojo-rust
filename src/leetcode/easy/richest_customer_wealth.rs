struct Solution {}

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|account| {
            account.iter().sum()
        }).fold(0, |a,b|{
            if a > b {
                a
            } else {
                b
            }
        })
    }
}