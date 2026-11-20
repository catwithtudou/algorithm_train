package leetcode

import "slices"

func maxSelectedElements(nums []int) (ans int) {
	slices.Sort(nums)
	f := map[int]int{}
	for _, x := range nums {
		f[x+1] = f[x] + 1
		f[x] = f[x-1] + 1
		ans = max(ans, max(f[x], f[x+1]))
	}
	return
}
