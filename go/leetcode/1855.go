package leetcode

func maxDistance1855(nums1 []int, nums2 []int) (ans int) {

	i := 0

	for j, y := range nums2 {
		for i < len(nums1) && nums1[i] > y {
			i++
		}
		if i == len(nums1) {
			break
		}
		ans = max(ans, j-i)
	}

	return
}
