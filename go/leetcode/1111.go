package leetcode

func maxDepthAfterSplit(seq string) []int {
	ans := make([]int, len(seq))
	depth := 0
	for i := range seq {
		if seq[i] == '(' {
			depth++
			ans[i] = depth & 1
		} else {
			// 先分组再减少深度，保证配对括号属于同一组。
			ans[i] = depth & 1
			depth--
		}
	}
	return ans
}
