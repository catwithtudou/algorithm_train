package leetcode

func lengthAfterTransformations(s string, t int) (ans int) {
	cnt := [26]int{}
	for _, c := range s {
		cnt[c-'a']++
	}

	for i := 0; i < t; i++ {
		nxt := [26]int{}
		nxt[0] = cnt[25]
		nxt[1] = (cnt[0] + cnt[25]) % mod
		for j := 2; j < 26; j++ {
			nxt[j] = cnt[j-1]
		}
		cnt = nxt
	}

	for i := 0; i < 26; i++ {
		ans = (ans + cnt[i]) % mod
	}

	return
}
