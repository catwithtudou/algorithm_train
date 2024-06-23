package leetcode

func temperatureTrend(temperatureA []int, temperatureB []int) int {
	same, ans := 0, 0
	for i := 1; i < len(temperatureA); i++ {
		a := temperatureA[i] - temperatureA[i-1]
		b := temperatureB[i] - temperatureB[i-1]
		if (a == 0 && b == 0) || a*b > 0 {
			same++
			ans = max(ans, same)
		} else {
			same = 0
		}
	}

	return ans
}
