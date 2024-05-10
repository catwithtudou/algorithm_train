package leetcode

func countTestedDevices(batteryPercentages []int) int {
	dec := 0
	for _, v := range batteryPercentages {
		if v-dec > 0 {
			dec++
		}
	}
	return dec
}
