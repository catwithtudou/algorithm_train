package leetcode

import "strings"

func isValidSerialization(preorder string) bool {
	var stk []string
	for _, s := range strings.Split(preorder, ",") {
		stk = append(stk, s)
		for len(stk) >= 3 && stk[len(stk)-1] == "#" && stk[len(stk)-2] == "#" && stk[len(stk)-3] != "#" {
			stk = stk[:len(stk)-3]
			stk = append(stk, "#")
		}

	}

	return len(stk) == 1 && stk[0] == "#"
}

func isValidSerialization331(preorder string) bool {
	diff := 1
	for _, s := range strings.Split(preorder, ",") {
		diff -= 1
		if diff < 0 {
			return false
		}
		if s != "#" {
			diff += 2
		}

	}

	return diff == 0
}
