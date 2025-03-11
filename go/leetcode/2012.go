package leetcode

func sumOfBeauties(nums []int) (ans int) {

	n := len(nums)

	sufMin := make([]int, n)
	sufMin[n-1] = nums[n-1]
	for i := n - 2; i > 1; i-- {
		sufMin[i] = min(sufMin[i+1], nums[i])
	}

	preMax := nums[0]
	for i := 1; i < n-1; i++ {
		x := nums[i]
		if preMax < x && x < sufMin[i+1] {
			ans += 2
		} else if nums[i-1] < x && x < nums[i+1] {
			ans++
		}
		preMax = max(preMax, x)
	}

	return
}
