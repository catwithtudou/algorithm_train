package leetcode

func waysToSplitArray(nums []int) (ans int) {
	total := 0
	for _, num := range nums {
		total += num
	}

	s := 0
	for i := 0; i < len(nums)-1; i++ {
		s += nums[i]
		if s*2 >= total {
			ans++
		}
	}
	return
}
