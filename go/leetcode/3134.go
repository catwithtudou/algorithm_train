package leetcode

import "sort"

func medianOfUniquenessArray(nums []int) int {
	n := len(nums)
	k := (n*(n+1)/2 + 1) / 2
	ans := 1 + sort.Search(n-1, func(upper int) bool {
		upper++
		cnt := 0
		l := 0
		freq := map[int]int{}
		for r, in := range nums {
			freq[in]++              // 移入右端点
			for len(freq) > upper { // 窗口内元素过多
				out := nums[l]
				freq[out]-- // 移出左端点
				if freq[out] == 0 {
					delete(freq, out)
				}
				l++
			}
			cnt += r - l + 1 // 右端点固定为 r 时，有 r-l+1 个合法左端点
			if cnt >= k {
				return true
			}
		}
		return false
	})
	return ans
}
