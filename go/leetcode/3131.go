package leetcode

func addedInteger(nums1 []int, nums2 []int) int {
	minNum1 := nums1[0]
	minNum2 := nums2[0]
	for i := 1; i < len(nums1); i++ {
		minNum1 = min(minNum1, nums1[i])
		minNum2 = min(minNum2, nums2[i])
	}
	return minNum2 - minNum1
}
