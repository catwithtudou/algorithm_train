package leetcode

func minimumPushes(word string) int {
	n := len(word)
	k := n / 8
	return (k*4 + n%8) * (k + 1)
}
