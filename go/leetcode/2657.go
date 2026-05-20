package leetcode

import "math/bits"

func findThePrefixCommonArray(A []int, B []int) []int {

	var p, q uint

	for i, x := range A {
		p |= 1 << x
		q |= 1 << B[i]
		A[i] = bits.OnesCount(p & q)

	}

	return A
}
