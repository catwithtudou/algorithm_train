package leetcode

func minimumDeletions2091(nums []int) int {
	n := len(nums)
	p, q := 0, 0
	for i, x := range nums {
		if x < nums[p] {
			p = i
		} else if x > nums[q] {
			q = i
		}
	}

	if p > q {
		p, q = q, p
	}

	return min(min(q+1, n-p), p+1+n-q)
}
