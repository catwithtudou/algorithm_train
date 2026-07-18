package leetcode

import "slices"

func findGCD(nums []int) int {
	return gcd(slices.Min(nums), slices.Max(nums))
}
