package leetcode

func earliestFinishTime3635(landStartTime []int, landDuration []int, waterStartTime []int, waterDuration []int) int {
	landWater := earliestFinishTimeSolve(landStartTime, landDuration, waterStartTime, waterDuration)
	waterLand := earliestFinishTimeSolve(waterStartTime, waterDuration, landStartTime, landDuration)
	return min(landWater, waterLand)
}
