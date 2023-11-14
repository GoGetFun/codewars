fn exp_sum(n: u64) -> u64 {
    let mut dp = vec![0; (n+1) as usize];
    dp[0] = 1;

    for num in 1..=n {
        for i in num..=n {
            dp[i as usize] += dp[(i-num) as usize];
        }
    }

    return dp[n as usize];
}