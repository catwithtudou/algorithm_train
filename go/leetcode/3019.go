package leetcode

func countKeyChanges(s string) (ans int) {

	for i := 1; i < len(s); i++ {
		if s[i]&31 != s[i-1]&31 {
			ans++
		}
	}

	return
}
