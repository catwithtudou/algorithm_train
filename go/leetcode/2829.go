package leetcode

func minimumSumKAvoidng(n int, k int) (ans int) {
	limitNum := make(map[int]struct{})
	for i, cnt := 1, 0; cnt < n; i++ {
		if _, ok := limitNum[i]; ok {
			continue
		}

		if i >= k {
			ans += i
			cnt++
			continue
		}

		limitNum[k-i] = struct{}{}
		ans += i
		cnt++
	}

	return
}
