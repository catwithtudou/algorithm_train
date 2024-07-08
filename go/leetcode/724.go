package leetcode

func pivotIndex(nums []int) int {
	sum := 0
	for _, v := range nums {
		sum += v
	}
	leftS := 0
	for i, x := range nums {
		if leftS*2 == sum-x {
			return i
		}
		leftS += x
	}
	return -1
}
