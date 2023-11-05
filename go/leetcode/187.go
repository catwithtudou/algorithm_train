package leetcode

const L = 10

func findRepeatedDnaSequences(s string) []string {
	strMap := make(map[string]int)
	ans := make([]string, 0, len(s))
	for i := 0; i <= len(s)-L; i++ {
		subStr := s[i : i+L]
		strMap[subStr]++
		if count := strMap[subStr]; count == 2 {
			ans = append(ans, subStr)
		}
	}
	return ans
}

// 滑动窗口+位运算解法
func findRepeatedDnaSequences_One(s string) (ans []string) {
	n := len(s)
	if n <= L {
		return
	}

	x := 0
	for _, ch := range s[:L-1] {
		x = x<<2 | bin[byte(ch)]
	}

	cnt := make(map[int]int, len(s))
	for i := 0; i <= n-L; i++ {
		x = (x<<2 | bin[s[i+L-1]]) & (1<<20 - 1)
		cnt[x]++
		if cnt[x] == 2 {
			ans = append(ans, s[i:i+L])
		}
	}
	return
}

var bin = map[byte]int{'A': 0, 'C': 1, 'G': 2, 'T': 3}
