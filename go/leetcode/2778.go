package leetcode

func sumOfSquares(nums []int) (ans int) {

	for i, x := range nums {
		if len(nums)%(i+1) != 0 {
			continue
		}
		ans += x * x
	}

	return
}
