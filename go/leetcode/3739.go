package leetcode

func countMajoritySubarraysII(nums []int, target int) (ans int64) {

	cnt := map[int]int{0: 1}

	s, f := 0, 0

	for _, x := range nums {
		if x == target {
			f += cnt[s]
			s++
		} else {
			s--
			f -= cnt[s]
		}
		ans += int64(f)
		cnt[s]++
	}

	return
}
