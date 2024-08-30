package leetcode

import "strconv"

func sumDigitDifferences(nums []int) int64 {
	var ans int64
	cnt := make([][10]int, len(strconv.Itoa(nums[0])))
	for k, n := range nums {
		for i := 0; n > 0; n /= 10 {
			d := n % 10
			ans += int64(k - cnt[i][d])
			cnt[i][d]++
			i++
		}
	}

	return ans
}
