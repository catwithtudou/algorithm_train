package leetcode

func maximumLength3020(nums []int) (ans int) {

	cnt := make(map[int]int)

	for _, v := range nums {
		cnt[v]++
	}

	ans = cnt[1] - 1 | 1

	delete(cnt, 1)

	for v := range cnt {
		res := 0
		for cnt[v] >= 2 {
			res += 2
			v *= v
		}
		res += cnt[v]
		ans = max(ans, res-1|1)
	}

	return ans
}
