package leetcode

func countGoodArrays(n int, m int, k int) int {
	const mod = 1_000_000_007
	const mx = 100_000

	var F, invF [mx]int

	F[0] = 1
	for i := 1; i < mx; i++ {
		F[i] = F[i-1] * i % mod
	}

	invF[mx-1] = quickPow(F[mx-1], mod-2, mod)
	for i := mx - 1; i > 0; i-- {
		invF[i-1] = invF[i] * i % mod
	}

	comb := func(n, m int) int {
		return F[n] * invF[m] % mod * invF[n-m] % mod
	}

	return comb(n-1, k) * m % mod * quickPow(m-1, n-k-1, mod) % mod
}
