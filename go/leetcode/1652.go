package leetcode

func decrypt(code []int, k int) []int {
	n := len(code)
	ans := make([]int, n)
	if k == 0 {
		return ans
	}
	if k > 0 {
		for i := range code {
			for j := 0; j < k; j++ {
				ans[i] += code[(i+j+1)%n]
			}
		}
		return ans
	}
	if k < 0 {
		for i := range code {
			for j := k; j < 0; j++ {
				ans[i] += code[(i+j+n)%n]
			}
		}
		return ans
	}
	return ans
}
