package leetcode

func maximumScore(nums []int, k int) int {

	n := len(nums)
	left := make([]int, n)
	var st []int

	for i, x := range nums {
		for len(st) > 0 && x <= nums[st[len(st)-1]] {
			st = st[:len(st)-1]
		}

		if len(st) > 0 {
			left[i] = st[len(st)-1]
		} else {
			left[i] = -1
		}
		st = append(st, i)
	}

	right := make([]int, n)
	st = st[:0]

	for i := n - 1; i >= 0; i-- {
		for len(st) > 0 && nums[i] <= nums[st[len(st)-1]] {
			st = st[:len(st)-1]
		}

		if len(st) > 0 {
			right[i] = st[len(st)-1]
		} else {
			right[i] = n
		}
		st = append(st, i)
	}

	ans := 0
	for i, h := range nums {
		l, r := left[i], right[i]
		if l < k && k < r {
			ans = max(ans, h*(r-l-1))
		}
	}
	return ans
}
