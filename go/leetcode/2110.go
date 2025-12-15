package leetcode

func getDescentPeriods(prices []int) (ans int64) {
	desc := 0
	for i, v := range prices {
		if i > 0 && v == prices[i-1]-1 {
			desc++
		} else {
			desc = 1
		}
		ans += int64(desc)
	}

	return
}
