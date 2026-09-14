package leetcode

func isRectangleOverlap(rec1 []int, rec2 []int) bool {
	internal := func(l1, r1, l2, r2 int) bool {
		return max(l1, l2) < min(r1, r2)
	}
	return internal(rec1[0], rec1[2], rec2[0], rec2[2]) && internal(rec1[1], rec1[3], rec2[1], rec2[3])
}
