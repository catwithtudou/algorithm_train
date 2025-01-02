package leetcode

import (
	"math"

	"github.com/emirpasic/gods/trees/redblacktree"
)

type MyCalendar struct {
	*redblacktree.Tree
}

func MyCalendarConstructor() MyCalendar {
	t := redblacktree.NewWithIntComparator()
	t.Put(math.MaxInt32, nil)
	return MyCalendar{t}
}

func (c *MyCalendar) Book(startTime int, endTime int) bool {
	node, _ := c.Ceiling(endTime)
	it := c.IteratorAt(node)
	if !it.Prev() || it.Value().(int) <= startTime {
		c.Put(startTime, endTime)
		return true
	}
	return false
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * obj := Constructor();
 * param_1 := obj.Book(startTime,endTime);
 */
