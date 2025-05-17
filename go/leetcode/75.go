package leetcode

func sortColors(nums []int) {
	p1, p0 := 0, 0
	for i, x := range nums {
		nums[i] = 2
		if x <= 1 {
			nums[p1] = 1
			p1++
		}
		if x == 0 {
			nums[p0] = 0
			p0++
		}
	}
	return
}
