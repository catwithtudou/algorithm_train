package leetcode

import "github.com/emirpasic/gods/trees/redblacktree"

type ExamRoom struct {
	rbt   *redblacktree.Tree
	left  map[int]int
	right map[int]int
	n     int
}

type ExamRoomPair struct {
	l, r int
}

func ConstructorExamRoom(n int) ExamRoom {
	dist := func(p ExamRoomPair) int {
		if p.l == -1 || p.r == n {
			return p.r - p.l - 1
		}
		return (p.r - p.l) >> 1
	}
	cmp := func(a, b any) int {
		x, y := a.(ExamRoomPair), b.(ExamRoomPair)
		d1, d2 := dist(x), dist(y)
		if d1 == d2 {
			return x.l - y.l
		}
		return d2 - d1
	}
	result := ExamRoom{redblacktree.NewWith(cmp), map[int]int{}, map[int]int{}, n}
	result.add(ExamRoomPair{-1, n})
	return result
}

func (this *ExamRoom) Seat() int {
	p := this.rbt.Left().Key.(ExamRoomPair)
	idx := (p.l + p.r) >> 1
	if p.l == -1 {
		idx = 0
	} else if p.r == this.n {
		idx = this.n - 1
	}
	this.del(p)
	this.add(ExamRoomPair{p.l, idx})
	this.add(ExamRoomPair{idx, p.r})
	return idx
}

func (this *ExamRoom) Leave(p int) {
	l, _ := this.left[p]
	r, _ := this.right[p]
	this.del(ExamRoomPair{l, p})
	this.del(ExamRoomPair{p, r})
	this.add(ExamRoomPair{l, r})
}

func (this *ExamRoom) add(p ExamRoomPair) {
	this.rbt.Put(p, struct{}{})
	this.left[p.r] = p.l
	this.right[p.l] = p.r
}

func (this *ExamRoom) del(p ExamRoomPair) {
	this.rbt.Remove(p)
	delete(this.left, p.r)
	delete(this.right, p.l)
}

/**
 * Your ExamRoom object will be instantiated and called as such:
 * obj := Constructor(n);
 * param_1 := obj.Seat();
 * obj.Leave(p);
 */
