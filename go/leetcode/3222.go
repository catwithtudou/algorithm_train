package leetcode

func losingPlayer(x int, y int) string {
	return []string{"Bob", "Alice"}[min(x, y/4)%2]
}
