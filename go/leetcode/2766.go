package leetcode

import "golang.org/x/exp/slices"

func relocateMarbles(nums []int, moveFrom []int, moveTo []int) []int {
	set := map[int]struct{}{}
	for _, x := range nums {
		set[x] = struct{}{}
	}

	for i, v := range moveFrom {
		delete(set, v)
		set[moveTo[i]] = struct{}{}
	}
	ans := make([]int, 0, len(nums))
	for k, _ := range set {
		ans = append(ans, k)
	}
	slices.Sort(ans)
	return ans
}
