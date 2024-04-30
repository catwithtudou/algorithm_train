package leetcode

func numberOfEmployeesWhoMetTarget(hours []int, target int) int {
	ans := 0
	for _, v := range hours {
		if v >= target {
			ans++
		}
	}

	return ans
}
