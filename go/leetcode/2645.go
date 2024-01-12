package leetcode

func addMinimum(word string) int {
	ans := int(word[0] - word[len(word)-1] + 2)
	for i := 1; i < len(word); i++ {
		ans += int((word[i] - word[i-1] + 2) % 3)
	}
	return ans
}

func addMinimumByOther(word string) int {
	t := 1
	for i := 1; i < len(word); i++ {
		if word[i-1] >= word[i] {
			t++
		}
	}
	return 3*t - len(word)
}
