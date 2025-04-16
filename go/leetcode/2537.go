package leetcode

func countGood(nums []int, k int) (ans int64) {
	cnt := make(map[int]int)
	pairs, left := 0, 0
	for _, v := range nums {
		pairs += cnt[v]
		cnt[v]++
		for pairs >= k {
			pairs -= cnt[nums[left]]
			cnt[nums[left]]--
			left++
		}
		ans += int64(left)
	}

	return
}
