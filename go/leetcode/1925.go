package leetcode

func countTriples(n int) (ans int) {
	for u := 3; u*u < n*2; u += 2 {
		for v := 1; v < u && u*u+v*v <= n*2; v += 2 {
			if gcd(u, v) == 1 {
				ans += n * 2 / (u*u + v*v)
			}
		}
	}
	return ans * 2
}
