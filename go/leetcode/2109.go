package leetcode

func addSpaces(s string, spaces []int) string {
	ans := make([]byte, 0, len(s)+len(spaces))

	j := 0

	for i, c := range s {
		if j < len(spaces) && i == spaces[j] {
			ans = append(ans, ' ')
			j++
		}
		ans = append(ans, byte(c))
	}

	return string(ans)
}
