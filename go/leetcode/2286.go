package leetcode

import "math/bits"

type seg []struct{ l, r, min, sum int }

func (s seg) build(o, l, r int) {
	s[o].l, s[o].r = l, r
	if l == r {
		return
	}
	m := (l + r) >> 1
	s.build(o<<1, l, m)
	s.build(o<<1|1, m+1, r)
}

func (s seg) update(o, i, val int) {
	if s[o].l == s[o].r {
		s[o].min += val
		s[o].sum += val
		return
	}

	m := (s[o].l + s[o].r) >> 1
	if i <= m {
		s.update(o<<1, i, val)
	} else {
		s.update(o<<1|1, i, val)
	}
	lo, ro := s[o<<1], s[o<<1|1]
	s[o].min = min(lo.min, ro.min)
	s[o].sum = lo.sum + ro.sum
	return
}

func (s seg) querySum(o, l, r int) (sum int) {
	if l <= s[o].l && r >= s[o].r {
		return s[o].sum
	}
	m := (s[o].l + s[o].r) >> 1
	if l <= m {
		sum = s.querySum(o<<1, l, r)
	}
	if r > m {
		sum += s.querySum(o<<1|1, l, r)
	}
	return
}

func (s seg) findFirst(o, r, val int) int {
	if s[o].min > val {
		return -1
	}
	if s[o].l == s[o].r {
		return s[o].l
	}

	m := (s[o].l + s[o].r) / 2
	if s[o*2].min <= val {
		return s.findFirst(o*2, r, val)
	}
	if r > m {
		return s.findFirst(o*2+1, r, val)
	}

	return -1
}

type BookMyShow struct {
	seg
	n, m int
}

func ConstructorBookMyShow(n int, m int) BookMyShow {
	s := make(seg, 2<<bits.Len(uint(n-1)))
	s.build(1, 0, n-1)
	return BookMyShow{s, n, m}
}

func (this *BookMyShow) Gather(k int, maxRow int) []int {
	r := this.findFirst(1, maxRow, this.m-k)
	if r < 0 {
		return nil
	}
	c := this.querySum(1, r, r)
	this.update(1, r, k)
	return []int{r, c}
}

func (this *BookMyShow) Scatter(k int, maxRow int) bool {
	s := this.querySum(1, 0, maxRow)
	if s > this.m*(maxRow+1)-k {
		return false
	}
	i := this.findFirst(1, maxRow, this.m-1)
	for k > 0 {
		left := min(this.m-this.querySum(1, i, i), k)
		this.update(1, i, left)
		k -= left
		i++
	}
	return true
}

/**
 * Your BookMyShow object will be instantiated and called as such:
 * obj := Constructor(n, m);
 * param_1 := obj.Gather(k,maxRow);
 * param_2 := obj.Scatter(k,maxRow);
 */
