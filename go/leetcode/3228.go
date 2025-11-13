package leetcode

func maxOperations3228(s string) (ans int) {
	cnt1 := 0
	for i := 0; i < len(s); i++ {
		if s[i] == '1' {
			cnt1++
			continue
		}
		for i+1 < len(s) && s[i+1] == '0' {
			i++
		}
		ans += cnt1
	}
	return
}
