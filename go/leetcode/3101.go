package leetcode

func countAlternatingSubarrays(nums []int) int64 {
	cnt, ans := 1, 1
	for i := 1; i < len(nums); i++ {
		if nums[i] != nums[i-1] {
			cnt++
		} else {
			cnt = 1
		}
		ans += cnt
	}
	return int64(ans)
}
