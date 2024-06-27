package leetcode

func smallestString(s string) string {
	t := []byte(s)
	for i, c := range t {
		if c <= 'a' {
			continue
		}

		for ; i < len(t) && t[i] > 'a'; i++ {
			t[i]--
		}
		return string(t)
	}
	t[len(t)-1] = 'z'
	return string(t)
}
