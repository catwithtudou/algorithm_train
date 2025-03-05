package leetcode

func breakPalindrome(palindrome string) string {
	n := len(palindrome)
	if n <= 1 {
		return ""
	}

	s := []byte(palindrome)
	for i := range n / 2 {
		if s[i] != 'a' {
			s[i] = 'a'
			return string(s)
		}
	}
	s[n-1] = 'b'
	return string(s)
}
