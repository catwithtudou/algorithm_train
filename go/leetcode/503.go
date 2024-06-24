package leetcode

func nextGreaterElements(nums []int) []int {
	n := len(nums)
	ans := make([]int, n)
	for i := range ans {
		ans[i] = -1
	}

	st := []int{}
	for i := 2*n - 1; i >= 0; i-- {
		x := nums[i%n]
		for len(st) > 0 && x >= st[len(st)-1] {
			st = st[:len(st)-1]
		}
		if i < n && len(st) > 0 {
			ans[i] = st[len(st)-1]
		}
		st = append(st, x)
	}

	return ans
}
