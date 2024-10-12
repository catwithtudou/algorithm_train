package leetcode

func duplicateNumbersXOR(nums []int) int {
	vis, ans := 0, 0
	for _, x := range nums {
		if vis>>x&1 > 0 {
			ans ^= x
		} else {
			vis |= 1 << x
		}
	}
	return ans
}
