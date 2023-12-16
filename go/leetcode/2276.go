package leetcode

import "github.com/emirpasic/gods/maps/treemap"

type CountIntervals struct {
	*treemap.Map
	cnt int
}

func NewConstructor() CountIntervals {
	return CountIntervals{
		treemap.NewWithIntComparator(), 0,
	}
}

func (this *CountIntervals) Add(left int, right int) {
	for k, v := this.Floor(right); k != nil && left <= v.(int); k, v = this.Floor(right) {
		l, r := k.(int), v.(int)
		left, right = min(left, l), max(right, r)
		this.cnt -= r - l + 1
		this.Remove(k)
	}
	this.cnt += right - left + 1
	this.Put(left, right)

}

func (this *CountIntervals) Count() int {
	return this.cnt
}
