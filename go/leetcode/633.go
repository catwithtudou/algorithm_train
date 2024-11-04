package leetcode

import "math"

func judgeSquareSum(c int) bool {
	for a := 0; a*a <= c/2; a++ {
		b := int(math.Sqrt(float64(c - a*a)))
		if a*a+b*b == c {
			return true
		}
	}
	return false
}
