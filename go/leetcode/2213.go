package leetcode

import "math/bits"

type segData struct{ mx, pre, suf int }

type segment struct {
	s []byte
	d []segData
}

func newSegment(s []byte) *segment {
	n := len(s)
	t := &segment{s, make([]segData, 2<<bits.Len(uint(n-1)))}
	t.build(1, 0, n-1)
	return t
}

func (t *segment) maintain(node, l, m, r int) {
	a, b := t.d[node*2], t.d[node*2+1]
	mx := max(a.mx, b.mx)
	pre := a.pre
	suf := b.suf
	if t.s[m] == t.s[m+1] {
		mx = max(mx, a.suf+b.pre)
		if pre == m-l+1 {
			pre += b.pre
		}
		if suf == r-m {
			suf += a.suf
		}
	}
	t.d[node] = segData{mx, pre, suf}
}

func (t *segment) build(node, l, r int) {
	if l == r { // 叶子
		t.d[node] = segData{1, 1, 1} // 初始化叶节点的值
		return
	}
	m := (l + r) >> 1
	t.build(node*2, l, m)     // 初始化左子树
	t.build(node*2+1, m+1, r) // 初始化右子树
	t.maintain(node, l, m, r)
}

func (t *segment) update(node, l, r, i int, ch byte) {
	if l == r { // 叶子（到达目标）
		t.s[i] = ch
		return
	}
	m := (l + r) >> 1
	if i <= m { // i 在左子树
		t.update(node*2, l, m, i, ch)
	} else { // i 在右子树
		t.update(node*2+1, m+1, r, i, ch)
	}
	t.maintain(node, l, m, r)
}

func longestRepeating(s string, queryCharacters string, queryIndices []int) []int {
	n := len(s)
	t := newSegment([]byte(s))
	ans := make([]int, len(queryIndices))
	for k, i := range queryIndices {
		t.update(1, 0, n-1, i, queryCharacters[k])
		ans[k] = t.d[1].mx
	}

	return ans
}
