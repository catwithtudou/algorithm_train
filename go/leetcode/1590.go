package leetcode

func minSubarray(nums []int, p int) int {
	n := len(nums)
	s := make([]int, n+1)
	for i, x := range nums {
		s[i+1] = (s[i] + x) % p
	}
	x := s[n]
	if x == 0 {
		return 0
	}

	ans := n
	last := make(map[int]int)
	for i, v := range s {
		last[v] = i
		if j, ok := last[(v-x+p)%p]; ok {
			ans = min(ans, i-j)
		}
	}

	if ans < n {
		return ans
	}

	return -1
}
