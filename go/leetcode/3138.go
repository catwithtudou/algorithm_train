package leetcode

func minAnagramLength(s string) int {
	n := len(s)
	for k := 1; k <= n/2; k++ {
		if n%k > 0 {
			continue
		}
		var cnt0 [26]int
		for _, c := range s[:k] {
			cnt0[c-'a']++
		}
		flag := false
		for i := k * 2; i <= n; i += k {
			var cnt1 [26]int
			for _, c := range s[i-k : i] {
				cnt1[c-'a']++
			}
			if cnt0 != cnt1 {
				flag = true
				break
			}
		}
		if flag {
			continue
		}

		return k
	}
	return n
}
