package leetcode

import "sort"

type snapPair struct {
	snapId, val int
}

type SnapshotArray struct {
	curSnapId int
	history   map[int][]snapPair
}

func ConstructorSnapshotArray(length int) SnapshotArray {
	return SnapshotArray{history: map[int][]snapPair{}}
}

func (this *SnapshotArray) Set(index int, val int) {
	this.history[index] = append(this.history[index], snapPair{this.curSnapId, val})
}

func (this *SnapshotArray) Snap() int {
	this.curSnapId++
	return this.curSnapId - 1
}

func (this *SnapshotArray) Get(index int, snapId int) int {
	h := this.history[index]
	i := sort.Search(len(h), func(i int) bool {
		return h[i].snapId >= snapId+1
	}) - 1
	if i >= 0 {
		return h[i].val
	}
	return 0
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * obj := Constructor(length);
 * obj.Set(index,val);
 * param_2 := obj.Snap();
 * param_3 := obj.Get(index,snap_id);
 */
