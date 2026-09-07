package leetcode

func distinctSubseqII(s string) int {
	const mod = 1_000_000_007
	n := len(s)
	f := make([][26]int, n+1)
	for i, b := range s {
		total := 0
		for _, v := range f[i] {
			total += v
		}
		f[i+1] = f[i]
		f[i+1][b-'a'] = 1 + total%mod
	}
	total := 0
	for _, v := range f[n] {
		total += v
	}
	return total % mod
}
