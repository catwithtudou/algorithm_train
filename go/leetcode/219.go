package leetcode

func containsNearbyDuplicate(nums []int, k int) bool {
	lastMap := make(map[int]int, 0)
	for i, x := range nums {
		if j, ok := lastMap[x]; ok && i-j <= k {
			return true
		}
		lastMap[x] = i
	}
	return false
}
