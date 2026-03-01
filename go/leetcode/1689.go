package leetcode

func minPartitions(n string) int {
	ans := rune(0)
	for _, c := range n {
		ans = maxRune(ans, c)
	}
	return int(ans - '0')
}

func maxRune(a, b rune) rune {
	if a > b {
		return a
	}
	return b
}
