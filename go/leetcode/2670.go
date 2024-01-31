package leetcode

func distinctDifferenceArray(nums []int) []int {
	st := map[int]struct{}{}
	sufCnt := make([]int, len(nums)+1)
	for i := len(nums) - 1; i > 0; i-- {
		st[nums[i]] = struct{}{}
		sufCnt[i] = len(st)
	}
	var res []int
	st = map[int]struct{}{}
	for i := 0; i < len(nums); i++ {
		st[nums[i]] = struct{}{}
		res = append(res, len(st)-sufCnt[i+1])
	}
	return res
}
