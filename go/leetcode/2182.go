package leetcode

func repeatLimitedString(s string, repeatLimit int) string {
	const charNum = 26
	count := make([]int, charNum)
	for _, c := range s {
		count[c-'a']++
	}

	ret := make([]byte, 0, len(s))
	var m int
	for i, j := charNum-1, charNum-2; i >= 0 && j >= 0; {
		switch {
		case count[i] == 0:
			i--
			m = 0
		case m < repeatLimit:
			count[i]--
			m++
			ret = append(ret, byte(i+'a'))
		case j >= i || count[j] == 0:
			j--
		default:
			count[j]--
			m = 0
			ret = append(ret, byte(j+'a'))

		}
	}

	return string(ret)
}
