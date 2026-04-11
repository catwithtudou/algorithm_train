package leetcode

func minimumDistanceII(nums []int) int {
	n := len(nums)
	last := make([]int, n+1)
	last2 := make([]int, n+1)

	for i := range last {
		last[i] = -n
		last2[i] = -n
	}

	ans := n

	for i, x := range nums {
		ans = min(ans, i-last2[x])
		last2[x] = last[x]
		last[x] = i
	}

	if ans == n {
		return -1
	}

	return ans * 2
}
