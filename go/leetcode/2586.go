package leetcode

var yuanyin = map[byte]bool{'a': true, 'e': true, 'i': true, 'o': true, 'u': true}

func vowelStrings(words []string, left int, right int) int {
	ans := 0
	for i := left; i <= right; i++ {
		startCh := words[i][0]
		endCh := words[i][len(words[i])-1]
		if yuanyin[startCh] && yuanyin[endCh] {
			ans++
		}
	}

	return ans
}
