package leetcode

func countValidSelections(nums []int) (ans int) {
	sum := 0
	for _, v := range nums {
		sum += v
	}
	preSum := 0
	for _, v := range nums {
		preSum += v
		if v == 0 {
			if preSum == (sum - preSum) {
				ans += 2
			} else if abs(preSum*2-sum) == 1 {
				ans++
			}
		}
	}

	return
}
