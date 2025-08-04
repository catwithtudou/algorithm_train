package leetcode

func totalFruit(fruits []int) (ans int) {

	left := 0
	cnt := make(map[int]int)

	for right, v := range fruits {
		cnt[v]++
		for len(cnt) > 2 {
			out := fruits[left]
			cnt[out]--
			if cnt[out] == 0 {
				delete(cnt, out)
			}
			left++
		}
		ans = max(ans, right-left+1)
	}

	return
}
