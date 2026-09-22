package leetcode

import "math/bits"

var k int

type arrayData struct {
	mul int
	cnt [5]int // 比 []int 快
}

type arrSeg []arrayData

func mergeData(a, b arrayData) arrayData {
	cnt := a.cnt
	for rx, c := range b.cnt {
		cnt[a.mul*rx%k] += c
	}
	return arrayData{a.mul * b.mul % k, cnt}
}

func newArrayData(val int) arrayData {
	mul := val % k
	cnt := [5]int{}
	cnt[mul] = 1
	return arrayData{mul, cnt}
}

func (t arrSeg) maintain(o int) {
	t[o] = mergeData(t[o<<1], t[o<<1|1])
}

func (t arrSeg) build(a []int, o, l, r int) {
	if l == r {
		t[o] = newArrayData(a[l])
		return
	}
	m := (l + r) >> 1
	t.build(a, o<<1, l, m)
	t.build(a, o<<1|1, m+1, r)
	t.maintain(o)
}

func (t arrSeg) update(o, l, r, i, val int) {
	if l == r {
		t[o] = newArrayData(val)
		return
	}
	m := (l + r) >> 1
	if i <= m {
		t.update(o<<1, l, m, i, val)
	} else {
		t.update(o<<1|1, m+1, r, i, val)
	}
	t.maintain(o)
}

func (t arrSeg) query(o, l, r, ql, qr int) arrayData {
	if ql <= l && r <= qr {
		return t[o]
	}
	m := (l + r) / 2
	if qr <= m {
		return t.query(o*2, l, m, ql, qr)
	}
	if ql > m {
		return t.query(o*2+1, m+1, r, ql, qr)
	}
	lRes := t.query(o*2, l, m, ql, qr)
	rRes := t.query(o*2+1, m+1, r, ql, qr)
	return mergeData(lRes, rRes)
}

func newSegmentTreeWithArray(a []int) arrSeg {
	n := len(a)
	t := make(arrSeg, 2<<bits.Len(uint(n-1)))
	t.build(a, 1, 0, n-1)
	return t
}

func resultArrayII(nums []int, K int, queries [][]int) []int {
	k = K
	t := newSegmentTreeWithArray(nums)
	n := len(nums)
	ans := make([]int, len(queries))
	for qi, q := range queries {
		t.update(1, 0, n-1, q[0], q[1])
		res := t.query(1, 0, n-1, q[2], n-1)
		ans[qi] = res.cnt[q[3]]
	}
	return ans
}
