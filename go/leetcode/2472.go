package leetcode

func isPalindromeStr(s string) bool {
	n := len(s)
	for i := range n / 2 {
		if s[i] != s[n-1-i] {
			return false
		}
	}
	return true
}

func maxPalindromes(s string, k int) int {
	n := len(s)
	f := make([]int, n+1)
	for i := k; i <= n; i++ {
		f[i] = f[i-1]
		if isPalindromeStr(s[i-k : i]) {
			f[i] = max(f[i], f[i-k]+1)
		}
		if i > k && isPalindromeStr(s[i-k-1:i]) {
			f[i] = max(f[i], f[i-k-1]+1)
		}
	}

	return f[n]
}
