package leetcode

import "golang.org/x/exp/slices"

func maxIncreasingCells(mat [][]int) int {
	type pair struct{ x, y int }
	g := make(map[int][]pair)
	keys := make([]int, 0)
	for i, row := range mat {
		for j, x := range row {
			if _, ok := g[x]; !ok {
				keys = append(keys, x)
			}
			g[x] = append(g[x], pair{i, j})
		}
	}
	slices.Sort(keys)

	rowMax := make([]int, len(mat))
	colMax := make([]int, len(mat[0]))

	for _, x := range keys {
		pos := g[x]
		fs := make([]int, len(pos))
		for i, p := range pos {
			fs[i] = max(rowMax[p.x], colMax[p.y]) + 1
		}
		for i, p := range pos {
			rowMax[p.x] = max(rowMax[p.x], fs[i])
			colMax[p.y] = max(colMax[p.y], fs[i])
		}
	}

	return slices.Max(rowMax)
}
