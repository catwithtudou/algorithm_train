package leetcode

func areSimilar(mat [][]int, k int) bool {
	n := len(mat)
	for _, row := range mat {
		for j, x := range row {
			if x != row[(j+k)%n] {
				return false
			}
		}
	}
	return true
}
