package leetcode

import "math/rand"

type Skiplist struct {
	head     *SkipListNode
	level    int
	maxLevel int
}

type SkipListNode struct {
	val   int
	next  []*SkipListNode
	level int
}

const SKIPLISTMAXLEVEL = 32      // 定义最大层数
const SKIPLISTPROBABILITY = 0.25 // 定义升层概率

func ConstructorSkiplist() Skiplist {
	return Skiplist{
		head: &SkipListNode{
			val:   -1,
			next:  make([]*SkipListNode, SKIPLISTMAXLEVEL),
			level: SKIPLISTMAXLEVEL,
		},
		level:    1,
		maxLevel: SKIPLISTMAXLEVEL,
	}
}

func (this *Skiplist) randomLevel() int {
	level := 1
	for level < SKIPLISTMAXLEVEL && rand.Float64() < SKIPLISTPROBABILITY {
		level++
	}
	return level
}

func (this *Skiplist) Search(target int) bool {
	current := this.head

	for i := this.level - 1; i >= 0; i-- {
		for current.next[i] != nil && current.next[i].val < target {
			current = current.next[i]
		}
	}
	current = current.next[0]
	return current != nil && current.val == target
}

func (this *Skiplist) Add(num int) {
	update := make([]*SkipListNode, SKIPLISTMAXLEVEL)
	currentNode := this.head

	for i := this.level - 1; i >= 0; i-- {
		for currentNode.next[i] != nil && currentNode.next[i].val < num {
			currentNode = currentNode.next[i]
		}
		update[i] = currentNode
	}

	level := this.randomLevel()
	for level > this.level {
		for i := this.level; i < level; i++ {
			update[i] = this.head
		}
		this.level = level
	}

	newNode := &SkipListNode{
		val:   num,
		next:  make([]*SkipListNode, level),
		level: level,
	}

	for i := 0; i < level; i++ {
		newNode.next[i] = update[i].next[i]
		update[i].next[i] = newNode
	}

	return
}

func (this *Skiplist) Erase(num int) bool {
	update := make([]*SkipListNode, SKIPLISTMAXLEVEL)
	current := this.head

	for i := this.level - 1; i >= 0; i-- {
		for current.next[i] != nil && current.next[i].val < num {
			current = current.next[i]
		}
		update[i] = current
	}

	current = current.next[0]
	if current == nil || current.val != num {
		return false
	}

	for i := 0; i < this.level; i++ {
		if update[i].next[i] != current {
			break
		}
		update[i].next[i] = current.next[i]
	}

	for this.level > 1 && this.head.next[this.level-1] == nil {
		this.level--
	}

	return true
}

/**
 * Your Skiplist object will be instantiated and called as such:
 * obj := Constructor();
 * param_1 := obj.Search(target);
 * obj.Add(num);
 * param_3 := obj.Erase(num);
 */
