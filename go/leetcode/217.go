package leetcode

func containsDuplicate(nums []int) bool {
	m := make(map[int]bool, len(nums))
	for _, num := range nums {
		if _, ok := m[num]; !ok {
			m[num] = true
		} else {
			return true
		}
	}
	return false
}
