package leetcode

func findRedundantConnection(edges [][]int) []int {
	parents := make([]int, len(edges)+1)
	for i := range parents {
		parents[i] = i
	}
	var find func(x int) int
	find = func(x int) int {
		if parents[x] != x {
			parents[x] = find(parents[x])
		}
		return parents[x]
	}
	var union func(from, to int) bool
	union = func(from, to int) bool {
		x, y := find(from), find(to)
		if x == y {
			return false
		}
		parents[x] = y
		return true
	}
	for _, e := range edges {
		if !union(e[0], e[1]) {
			return e
		}
	}
	return nil
}
