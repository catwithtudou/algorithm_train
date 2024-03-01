package leetcode

func validPartition(nums []int) bool {
	n := len(nums)
	f := make([]bool, n+1)
	f[0] = true
	for i := 1; i < n; i++ {

		if f[i-1] && nums[i] == nums[i-1] {
			f[i+1] = true
			continue
		}

		if i > 1 && f[i-2] && ((nums[i] == nums[i-1] && nums[i] == nums[i-2]) || (nums[i] == nums[i-1]+1 && nums[i-1] == nums[i-2]+1)) {
			f[i+1] = true
			continue
		}

	}

	return f[n]
}
