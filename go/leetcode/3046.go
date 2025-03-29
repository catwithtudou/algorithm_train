package leetcode

func isPossibleToSplit(nums []int) bool {
	cnt := map[int]int{}
	for _, x := range nums {
		cnt[x]++

		if cnt[x] > 2 {

			return false
		}
	}
	return true
}
