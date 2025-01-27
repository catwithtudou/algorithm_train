package leetcode

func jumpII(nums []int) (ans int) {
	curR, nextR := 0, 0
	for i, num := range nums[:len(nums)-1] {
		nextR = max(nextR, i+num)
		if i == curR {
			curR = nextR
			ans++
		}
	}
	return
}
