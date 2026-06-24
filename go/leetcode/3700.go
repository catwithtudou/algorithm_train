package leetcode

type cMatrix [][]int

func newCMatrix(n, m int) cMatrix {
	a := make(cMatrix, n)

	for i := range a {
		a[i] = make([]int, m)
	}

	return a
}

func (a cMatrix) mul(b cMatrix) cMatrix {
	c := newCMatrix(len(a), len(b[0]))

	for i, row := range a {
		for k, x := range row {
			if x == 0 {
				continue
			}

			for j, y := range b[k] {
				c[i][j] = (c[i][j] + x*y) % mod
			}
		}
	}

	return c
}

func (a cMatrix) powMul(n int, f1 cMatrix) cMatrix {
	res := f1
	for ; n > 0; n /= 2 {
		if n%2 > 0 {
			res = a.mul(res)
		}
		a = a.mul(a)
	}
	return res
}

func zigZagArraysII(n int, l int, r int) (ans int) {
	k := r - l + 1

	m := newCMatrix(k*2, k*2)

	for i := range k {
		for j := range i {
			m[i][k+j] = 1
		}
		for j := i + 1; j < k; j++ {
			m[k+i][j] = 1
		}
	}

	f1 := newCMatrix(k*2, 1)

	for i := range f1 {
		f1[i][0] = 1
	}

	fn := m.powMul(n-1, f1)
	for _, row := range fn {
		ans += row[0]
	}

	return ans % mod
}
