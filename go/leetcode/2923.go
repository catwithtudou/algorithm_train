package leetcode

func findChampion(grid [][]int) int {
next:
	for i, row := range grid {
		for j, x := range row {
			if i != j && x == 0 {
				continue next
			}
		}
		return i
	}
	panic(-1)
}
