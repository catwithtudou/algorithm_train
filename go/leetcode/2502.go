package leetcode

import "github.com/emirpasic/gods/trees/redblacktree"

type Allocator struct {
	rbt    *redblacktree.Tree
	midMap map[int][]int
}

func ConstructorAllocator(n int) Allocator {
	rbt := redblacktree.NewWithIntComparator()
	rbt.Put(-1, -1)
	rbt.Put(n, n)
	return Allocator{
		rbt:    rbt,
		midMap: make(map[int][]int),
	}

}

func (this *Allocator) Allocate(size int, mID int) int {
	lastEnd := -1
	it := this.rbt.Iterator()
	for it.Next() {
		if lastEnd != -1 {
			if it.Key().(int)-lastEnd >= size {
				this.rbt.Put(lastEnd, lastEnd+size-1)
				this.midMap[mID] = append(this.midMap[mID], lastEnd)
				return lastEnd
			}
		}
		lastEnd = it.Value().(int) + 1
	}

	return -1
}

func (this *Allocator) FreeMemory(mID int) (ans int) {
	for _, start := range this.midMap[mID] {
		if end, ok := this.rbt.Get(start); ok {
			this.rbt.Remove(start)
			ans += end.(int) - start + 1
		}
	}
	this.midMap[mID] = []int{}
	return
}

/**
 * Your Allocator object will be instantiated and called as such:
 * obj := Constructor(n);
 * param_1 := obj.Allocate(size,mID);
 * param_2 := obj.FreeMemory(mID);
 */
