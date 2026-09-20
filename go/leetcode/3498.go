package leetcode

func reverseDegree(s string) (ans int) {
	for i, c := range s {
		ans += int('{'-c) * (i + 1)
	}
	return
}
