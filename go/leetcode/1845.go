package leetcode

import (
	"container/heap"
	"sort"
)

type SeatManager struct {
	sort.IntSlice
	meats int
}

func ConstructorSeatManager(n int) SeatManager {
	return SeatManager{}
}

func (this *SeatManager) Reserve() int {
	if len(this.IntSlice) > 0 {
		return heap.Pop(this).(int)
	}
	this.meats += 1
	return this.meats
}

func (this *SeatManager) Unreserve(seatNumber int) {
	heap.Push(this, seatNumber)
}

func (this *SeatManager) Push(v any) {
	this.IntSlice = append(this.IntSlice, v.(int))
}

func (this *SeatManager) Pop() any {
	a := this.IntSlice
	v := a[len(a)-1]
	this.IntSlice = a[:len(a)-1]
	return v
}

/**
 * Your SeatManager object will be instantiated and called as such:
 * obj := Constructor(n);
 * param_1 := obj.Reserve();
 * obj.Unreserve(seatNumber);
 */
