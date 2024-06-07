package leetcode

func maxOperations(nums []int) int {
	cnt := nums[0] + nums[1]
	ans := 1
	for i := 3; i < len(nums) && nums[i-1]+nums[i] == cnt; i += 2 {
		ans++
	}
	return ans
}
