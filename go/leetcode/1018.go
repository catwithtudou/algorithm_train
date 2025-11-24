package leetcode

func prefixesDivBy5(nums []int) []bool {
	ans := make([]bool, len(nums))
	pre := 0
	for i, num := range nums {
		pre = (pre<<1 | num) % 5
		ans[i] = pre == 0
	}
	return ans
}
