package leetcode

func maxProduct(words []string) int {
	bitSlice := make([]int, len(words))
	for i, word := range words {
		for _, ch := range word {
			bitSlice[i] |= 1 << (ch - 'a')
		}
	}

	ans := 0
	//for i := 0; i < len(words)-1; i++ {
	//	for j := i + 1; j < len(words); j++ {
	//		if bitSlice[i]&bitSlice[j] == 0 {
	//			ans = max(ans, len(words[i])*len(words[j]))
	//		}
	//	}
	//}
	for i, x := range bitSlice {
		for j, y := range bitSlice[:i] {
			if x&y == 0 {
				ans = max(ans, len(words[i])*len(words[j]))
			}
		}
	}
	return ans
}
