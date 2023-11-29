package leetcode

import "github.com/emirpasic/gods/sets/treeset"

type SmallestInfiniteSet struct {
	index int
	coll  *treeset.Set
}

func NewSmallestInfiniteSet() SmallestInfiniteSet {
	return SmallestInfiniteSet{
		index: 1,
		coll:  treeset.NewWithIntComparator(),
	}
}

func (this *SmallestInfiniteSet) PopSmallest() int {
	if this.coll.Empty() {
		ans := this.index
		this.index++
		return ans
	}

	collNext := this.coll.Iterator()
	collNext.Next()
	ans := collNext.Value().(int)
	this.coll.Remove(ans)
	return ans
}

func (this *SmallestInfiniteSet) AddBack(num int) {
	if num < this.index {
		this.coll.Add(num)
	}
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * obj := Constructor();
 * param_1 := obj.PopSmallest();
 * obj.AddBack(num);
 */
