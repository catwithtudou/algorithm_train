package leetcode

func divisibilityArray(word string, m int) []int {
	ans := make([]int, len(word))
	cur := 0
	for i, w := range word {
		cur = (cur*10 + int(w-'0')) % m
		if cur == 0 {
			ans[i] = 1
		}
	}

	return ans
}
