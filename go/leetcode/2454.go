package leetcode

func secondGreaterElement(nums []int) []int {
	n := len(nums)
	ans := make([]int, n)
	for i := range ans {
		ans[i] = -1
	}

	var s, t []int
	for i, x := range nums {
		for len(t) > 0 && x > nums[t[len(t)-1]] {
			ans[t[len(t)-1]] = x
			t = t[:len(t)-1]
		}
		j := len(s) - 1
		for j >= 0 && x > nums[s[j]] {
			j--
		}
		t = append(t, s[j+1:]...)
		s = append(s[:j+1], i)
	}

	return ans
}
