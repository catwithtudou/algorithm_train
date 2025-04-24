package leetcode

func countCompleteSubarrays(nums []int) (ans int) {

	set := map[int]struct{}{}
	for _, v := range nums {
		set[v] = struct{}{}
	}
	k := len(set)

	cnt := map[int]int{}
	left := 0
	for _, v := range nums {
		ans += left
		cnt[v]++
		for len(cnt) == k {
			ans++
			out := nums[left]
			cnt[out]--
			if cnt[out] == 0 {
				delete(cnt, out)
			}
			left++
		}
	}

	return
}
