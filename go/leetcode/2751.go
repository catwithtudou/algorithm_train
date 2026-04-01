package leetcode

import "slices"

func survivedRobotsHealths(positions []int, healths []int, directions string) (ans []int) {
	idx := make([]int, len(positions))
	for i := range idx {
		idx[i] = i
	}

	slices.SortFunc(idx, func(i, j int) int {
		return positions[i] - positions[j]
	})

	st := []int{}
	for _, i := range idx {
		if directions[i] == 'R' {
			st = append(st, i)
			continue
		}

		for len(st) > 0 {
			j := st[len(st)-1]

			if healths[j] > healths[i] {
				healths[i] = 0
				healths[j]--
				break
			} else if healths[i] == healths[j] {
				healths[i] = 0
				healths[j] = 0
				st = st[:len(st)-1]
				break
			}

			healths[i]--
			healths[j] = 0
			st = st[:len(st)-1]
		}
	}

	for _, h := range healths {
		if h > 0 {
			ans = append(ans, h)
		}
	}

	return ans
}
