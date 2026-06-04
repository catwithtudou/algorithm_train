package leetcode

import "strconv"

func totalWaviness(num1 int, num2 int) int {

	ans := 0

	for i := num1; i <= num2; i++ {
		ans += getCommonWave(i)
	}

	return ans
}

func getCommonWave(x int) int {
	str := strconv.Itoa(x)
	wave := 0

	for i := 1; i < len(str)-1; i++ {
		isPeak := str[i] > str[i-1] && str[i] > str[i+1]
		isBottom := str[i] < str[i-1] && str[i] < str[i+1]
		if isPeak || isBottom {
			wave++
		}
	}

	return wave
}
