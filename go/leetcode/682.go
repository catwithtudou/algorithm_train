package leetcode

import "strconv"

func calPoints(operations []string) int {
	st := []int{}
	for _, op := range operations {
		switch op[0] {
		case '+':
			st = append(st, st[len(st)-2]+st[len(st)-1])
		case 'D':
			st = append(st, st[len(st)-1]*2)
		case 'C':
			st = st[:len(st)-1]
		default:
			x, _ := strconv.Atoi(op)
			st = append(st, x)
		}
	}
	ans := 0
	for _, v := range st {
		ans += v
	}
	return ans
}
