package leetcode

func calc(nums []int) (sum int64, zero bool) {
	for _, x := range nums {
		if x == 0 {
			zero = true
			sum++
		} else {
			sum += int64(x)
		}
	}
	return
}

func minSum(nums1 []int, nums2 []int) int64 {
	s1, zero1 := calc(nums1)
	s2, zero2 := calc(nums2)
	if !zero1 && s1 < s2 || !zero2 && s2 < s1 {
		return -1
	}
	return maxInt64(s1, s2)
}
