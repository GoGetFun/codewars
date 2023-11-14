package kata

func ExpSum(n uint64) uint64 {
    dp := make([]uint64, n+1)
    dp[0] = 1

    for num := uint64(1); num <= n; num++ {
        for i := num; i <= n; i++ {
            dp[i] += dp[i-num]
        }
    }

    return dp[n]
}