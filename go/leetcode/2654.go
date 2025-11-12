package leetcode

func minOperations2654(nums []int) int {
	n, gcdAll, cnt1 := len(nums), 0, 0
	for _, x := range nums {
		gcdAll = gcd(gcdAll, x)
		if x == 1 {
			cnt1++
		}
	}

	if gcdAll > 1 {
		return -1
	}

	if cnt1 > 0 {
		return n - cnt1
	}

	minSize := n
	for i := range nums {
		g := 0
		for j, x := range nums[i:] {
			g = gcd(g, x)
			if g == 1 {
				minSize = min(minSize, j)
				break
			}
		}
	}
	return minSize + n - 1
}
