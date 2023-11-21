package leetcode

func minDeletion(nums []int) int {
	check := true
	ans := 0
	for i := 0; i+1 < len(nums); i++ {
		if nums[i] == nums[i+1] && check {
			ans++
			continue
		}
		check = !check
	}

	if (len(nums)-ans)%2 != 0 {
		ans++
	}
	return ans
}
