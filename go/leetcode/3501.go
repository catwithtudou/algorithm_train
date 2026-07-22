package leetcode

import (
	"math/bits"
	"sort"
)

type pairLr struct {
	l, r int
}

type STLr [][]int

func newSTLr(a []pairLr) STLr {
	n := len(a) - 1
	w := bits.Len(uint(n))
	st := make(STLr, w)

	for i := range st {
		st[i] = make([]int, n)
	}

	for j, p := range a[:n] {
		st[0][j] = p.r - p.l + a[j+1].r - a[j+1].l
	}

	for i := 1; i < w; i++ {
		for j := range n - 1<<i + 1 {
			st[i][j] = max(st[i-1][j], st[i-1][j+1<<(i-1)])
		}
	}

	return st
}

func (st STLr) query(l, r int) int {
	if l >= r {
		return 0
	}

	k := bits.Len(uint(r-l)) - 1
	return max(st[k][l], st[k][r-1<<k])
}

func maxActiveSectionsAfterTradeII(s string, queries [][]int) []int {
	n := len(s)

	total1 := 0

	a := []pairLr{{-1, -1}}

	start := 0
	for i := range n {
		if i == n-1 || s[i] != s[i+1] {
			if s[i] == '1' {
				total1 += i - start + 1
			} else {
				a = append(a, pairLr{start, i + 1})
			}
			start = i + 1
		}
	}
	a = append(a, pairLr{n + 1, n + 1})

	merge := func(x, y int) int {
		if x > 0 && y > 0 {
			return x + y
		}
		return 0
	}

	st := newSTLr(a)
	m := len(a)
	ans := make([]int, len(queries))
	for qi, q := range queries {
		ql, qr := q[0], q[1]+1
		i := sort.Search(m, func(i int) bool { return a[i].l >= ql })
		j := sort.Search(m, func(i int) bool { return a[i].r > qr }) - 1
		mx := 0
		if i <= j {
			mx = max(st.query(i, j),
				max(merge(a[i-1].r-ql, a[i].r-a[i].l),
					merge(qr-a[j+1].l, a[j].r-a[j].l)))
		} else if i == j+1 {
			mx = merge(a[i-1].r-ql, qr-a[j+1].l)
		}
		ans[qi] = total1 + mx
	}

	return ans
}
