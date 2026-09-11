package leetcode

func totalNumbers(digits []int) int {
	set := make(map[int]struct{})

	for i, x := range digits {
		if x%2 > 0 {
			continue
		}
		for j, y := range digits {
			if i == j {
				continue
			}
			for k, z := range digits {
				if z == 0 || k == j || k == i {
					continue
				}
				set[100*z+10*y+x] = struct{}{}
			}
		}
	}

	return len(set)
}
