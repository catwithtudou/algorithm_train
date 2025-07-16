package leetcode

func maximumLength3201(nums []int) (res int) {
	patterns := [][]int{{0, 0}, {0, 1}, {1, 0}, {1, 1}}
	for _, pattern := range patterns {
		cnt := 0
		for _, num := range nums {
			if num%2 == pattern[cnt%2] {
				cnt++
			}
		}
		res = max(res, cnt)
	}
	return
}
