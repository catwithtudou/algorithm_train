package leetcode

import "strings"

func countCollisions(directions string) int {
	s := strings.TrimLeft(directions, "L")
	s = strings.TrimRight(s, "R")
	return len(s) - strings.Count(s, "S")
}
