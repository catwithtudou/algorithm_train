package leetcode

func makeSmallestPalindrome(s string) string {
	n := len(s) - 1
	t := []byte(s)
	for i := 0; i <= n/2; i++ {
		if s[i] == s[n-i] {
			continue
		}
		t[i] = minByte(t[i], t[n-i])
		t[n-i] = t[i]
	}

	return string(t)
}

func minByte(a, b byte) byte {
	if a > b {
		return b
	}
	return a
}
