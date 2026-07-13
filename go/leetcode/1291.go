package leetcode

import "golang.org/x/exp/slices"

func sequentialDigits(low int, high int) (ans []int) {

	for d := 1; d <= 9; d++ {

		x := d

		for i := d; i <= 9 && x <= high; i++ {
			if x >= low {
				ans = append(ans, x)
			}
			x = x*10 + i + 1
		}
	}

	slices.Sort(ans)

	return ans
}
