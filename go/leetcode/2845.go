package leetcode

func countInterestingSubarrays(nums []int, modulo int, k int) (ans int64) {
	sum := make([]int, len(nums)+1)
	for i, v := range nums {
		sum[i+1] = sum[i]
		if v%modulo == k {
			sum[i+1]++
		}
	}

	cnt := make([]int, min(modulo, len(nums)+1))
	for _, s := range sum {
		if s >= k {
			ans += int64(cnt[(s-k)%modulo])
		}
		cnt[s%modulo]++
	}

	return
}
