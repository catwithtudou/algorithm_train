package leetcode

func countPoints(rings string) int {
	n := len(rings)
	rod := [10]int{}
	for i := 0; i < n; i += 2 {
		char := rings[i]
		rodIdx := rings[i+1] - '0'
		if char == 'R' {
			rod[rodIdx] |= 1
		} else if char == 'B' {
			rod[rodIdx] |= 2
		} else {
			rod[rodIdx] |= 4
		}
	}

	res := 0
	for _, v := range rod {
		if v == 7 {
			res++
		}
	}

	return res
}

func countPoints_One(rings string) int {
	n := len(rings)
	rod := [10][3]int{}
	for i := 0; i < n; i += 2 {
		rod[rings[i+1]-'0'][getColorId(rings[i])]++
	}
	res := 0
	for _, v := range rod {
		for _, vv := range v {
			if vv <= 0 {
				res -= 1
				break
			}
		}
		res++
	}

	return res
}

func getColorId(color byte) int {
	if color == 'R' {
		return 0
	} else if color == 'G' {
		return 1
	}
	return 2
}
