package leetcode

func differenceOfSum(nums []int) (ans int) {
	for _, x := range nums {
		ans += x
		for x > 0 {
			ans -= x % 10
			x /= 10
		}
	}
	return ans
}
