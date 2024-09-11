package leetcode

func maximizeWin(prizePositions []int, k int) int {
	n := len(prizePositions)
	if 2*k+1 >= prizePositions[n-1]-prizePositions[0] {
		return n
	}

	mx := make([]int, n+1)
	left, ans := 0, 0
	for right, p := range prizePositions {
		for p-prizePositions[left] > k {
			left++
		}
		ans = max(ans, mx[left]+right-left+1)
		mx[right+1] = max(mx[right], right-left+1)
	}

	return ans
}
