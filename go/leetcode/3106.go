package leetcode

func getSmallestString(s string, k int) string {
	t := []byte(s)
	for i, c := range t {
		dis := min(int(c-'a'), int('z'-c+1))
		if dis > k {
			t[i] -= byte(k)
			break
		}
		t[i] = 'a'
		k -= dis
	}
	return string(t)
}
