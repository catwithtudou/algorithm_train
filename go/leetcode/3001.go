package leetcode

func minMovesToCaptureTheQueen(a int, b int, c int, d int, e int, f int) int {
	isBetween := func(l, m, r int) bool {
		return min(l, r) < m && m < max(l, r)
	}

	if a == e && (c != e || !isBetween(b, d, f)) {
		return 1
	}
	if b == f && (d != f || !isBetween(a, c, e)) {
		return 1
	}

	if c+d == e+f && (a+b != c+d || !isBetween(c, a, e)) {
		return 1
	}

	if c-d == e-f && (a-b != c-d || !isBetween(c, a, e)) {
		return 1
	}

	return 2
}
