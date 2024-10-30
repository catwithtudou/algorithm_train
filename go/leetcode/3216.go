package leetcode

func getSmallestString3216(s string) string {
	t := []byte(s)
	for i := 1; i < len(t); i++ {
		x, y := t[i-1], t[i]
		if x > y && (x%2 == y%2) {
			t[i-1], t[i] = y, x
			break
		}
	}
	return string(t)
}
