package leetcode

import (
	"github.com/emirpasic/gods/v2/trees/redblacktree"
)

type NumberContainers struct {
	indexToNumber   map[int]int
	numberToIndices map[int]*redblacktree.Tree[int, struct{}]
}

func NumberContainersConstructor() NumberContainers {
	return NumberContainers{
		indexToNumber:   make(map[int]int),
		numberToIndices: make(map[int]*redblacktree.Tree[int, struct{}]),
	}
}

func (n *NumberContainers) Change(index int, number int) {
	if oldNumber, ok := n.indexToNumber[index]; ok {
		n.numberToIndices[oldNumber].Remove(index)
	}

	n.indexToNumber[index] = number
	if n.numberToIndices[number] == nil {
		n.numberToIndices[number] = redblacktree.New[int, struct{}]()
	}
	n.numberToIndices[number].Put(index, struct{}{})
}

func (n *NumberContainers) Find(number int) int {
	indices, ok := n.numberToIndices[number]
	if !ok || indices.Size() == 0 {
		return -1
	}
	return indices.Left().Key
}

/**
 * Your NumberContainers object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Change(index,number);
 * param_2 := obj.Find(number);
 */
