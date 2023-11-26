package leetcode

func uniqueLetterString(s string) int {
	ans := 0
	d := make([][]int, 26)
	for i := range d {
		d[i] = []int{-1}
	}
	for k, v := range s {
		d[v-'A'] = append(d[v-'A'], k)
	}

	for _, v := range d {
		v = append(v, len(s))
		for i := 1; i < len(v)-1; i++ {
			ans += (v[i] - v[i-1]) * (v[i+1] - v[i])
		}

	}

	return ans
}
