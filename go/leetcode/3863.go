package leetcode

import "golang.org/x/exp/slices"

func minOperations3863(s string) int {
	t := []byte(s)

	if slices.IsSorted(t) {
		return 0
	}

	n := len(t)
	if n == 2 {
		return -1
	}

	mn := slices.Min(t)
	mx := slices.Max(t)

	if t[0] == mn || t[n-1] == mx {
		return 1
	}

	for _, ch := range t[1 : n-1] {
		if ch == mn || ch == mx {
			return 2
		}
	}

	return 3
}
