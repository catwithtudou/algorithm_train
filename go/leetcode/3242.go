package leetcode

var dirsNeighborSum = []struct{ x, y int }{{-1, 0}, {1, 0}, {0, -1}, {0, 1}, {1, 1}, {-1, 1}, {-1, -1}, {1, -1}}

type NeighborSum [][2]int

func ConstructorNeighborSum(grid [][]int) NeighborSum {
	n := len(grid)
	s := make(NeighborSum, n*n)
	for i, row := range grid {
		for j, v := range row {
			for k, d := range dirsNeighborSum {
				x, y := i+d.x, j+d.y
				if 0 <= x && x < n && 0 <= y && y < n {
					s[v][k/4] += grid[x][y]
				}
			}
		}
	}
	return s
}

func (this NeighborSum) AdjacentSum(value int) int {
	return this[value][0]
}

func (this NeighborSum) DiagonalSum(value int) int {
	return this[value][1]
}
