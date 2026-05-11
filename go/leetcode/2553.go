package leetcode

import "strconv"

func separateDigits(nums []int) (ans []int) {

	for _, x := range nums {
		for _, ch := range strconv.Itoa(x) {
			ans = append(ans, int(ch-'0'))
		}
	}

	return ans
}
