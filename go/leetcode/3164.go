package leetcode

func numberOfPairs3164(nums1 []int, nums2 []int, k int) (ans int64) {
	cnt := make(map[int]int64, 0)
	for _, x := range nums1 {
		if x%k != 0 {
			continue
		}
		x /= k
		for d := 1; d*d <= x; d++ {
			if x%d != 0 {
				continue
			}
			cnt[d] += 1
			if d*d < x {
				cnt[x/d] += 1
			}
		}
	}

	for _, v := range nums2 {
		ans += cnt[v]
	}
	return
}
