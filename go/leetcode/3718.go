package leetcode

func missingMultiple(nums []int, k int) int {
	hashMap := make(map[int]bool)
	for _, x := range nums {
		hashMap[x] = true
	}
	for x := k; ; x += k {
		if !hashMap[x] {
			return x
		}
	}
}
