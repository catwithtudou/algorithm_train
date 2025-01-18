package leetcode

func maxValue(nums []int, k int) (ans int) {

	findOrs := func(nums []int, k int) []map[int]bool {
		dp := make([]map[int]bool, 0)
		prev := make([]map[int]bool, k+1)
		for i := 0; i <= k; i++ {
			prev[i] = make(map[int]bool)
		}
		prev[0][0] = true

		for i := 0; i < len(nums); i++ {
			for j := min(k-1, i+1); j >= 0; j-- {
				for x := range prev[j] {
					prev[j+1][x|nums[i]] = true
				}
			}
			current := make(map[int]bool)
			for key := range prev[k] {
				current[key] = true
			}
			dp = append(dp, current)
		}

		return dp
	}

	reverse := func(nums []int) {
		for i, j := 0, len(nums)-1; i < j; i, j = i+1, j-1 {
			nums[i], nums[j] = nums[j], nums[i]
		}
	}

	left := findOrs(nums, k)
	reverse(nums)
	right := findOrs(nums, k)

	for i := k - 1; i < len(nums)-k; i++ {
		for l := range left[i] {
			for r := range right[len(nums)-i-2] {
				if l^r > ans {
					ans = l ^ r
				}
			}
		}
	}

	return

}
