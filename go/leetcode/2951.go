package leetcode

func findPeaks(mountain []int) []int {
	ans := make([]int, 0)
	for i := 1; i < len(mountain)-1; i++ {
		if mountain[i] > mountain[i-1] && mountain[i] > mountain[i+1] {
			ans = append(ans, i)
		}
	}
	return ans
}
