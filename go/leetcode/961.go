package leetcode

func repeatedNTimes(nums []int) int {
	cnt := make(map[int]bool)
	for _, v := range nums {
		_, ok := cnt[v]
		if ok {
			return v
		}
		cnt[v] = true
	}
	return -1
}
