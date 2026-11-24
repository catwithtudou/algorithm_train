package leetcode

import "slices"

func maxLength(nums []int) (ans int) {

	mx := slices.Max(nums)

	allLcm := 1

	for _, x := range nums {
		allLcm = lcm(allLcm, x)
	}

	for i := range nums {
		m, l, g := 1, 1, 0

		for j := i; j < len(nums) && m <= allLcm*mx; j++ {
			x := nums[j]
			m *= x
			l = lcm(l, x)
			g = gcd(g, x)
			if m == l*g {
				ans = max(ans, j-i+1)
			}
		}
	}

	return
}
