package leetcode

import "math"

func earliestFinishTimeSolve(landStartTime []int, landDuration []int, waterStartTime []int, waterDuration []int) int {
	minFinish := math.MaxInt

	for i, start := range landStartTime {
		minFinish = min(minFinish, start+landDuration[i])
	}

	res := math.MaxInt

	for i, start := range waterStartTime {
		res = min(res, max(start, minFinish)+waterDuration[i])
	}

	return res
}

func earliestFinishTime(landStartTime []int, landDuration []int, waterStartTime []int, waterDuration []int) int {
	landWater := earliestFinishTimeSolve(landStartTime, landDuration, waterStartTime, waterDuration)
	waterLand := earliestFinishTimeSolve(waterStartTime, waterDuration, landStartTime, landDuration)
	return min(landWater, waterLand)
}
