package leetcode

import "container/list"

type FrontMiddleBackQueue struct {
	left, right *list.List
}

func NewFrontMiddleBackQueue() FrontMiddleBackQueue {
	return FrontMiddleBackQueue{
		left:  list.New(),
		right: list.New(),
	}
}

func (f *FrontMiddleBackQueue) balance() {
	if f.left.Len() > f.right.Len() {
		f.right.PushFront(f.left.Remove(f.left.Back()))
	} else if f.right.Len() > f.left.Len()+1 {
		f.left.PushBack(f.right.Remove(f.right.Front()))
	}
}

func (f *FrontMiddleBackQueue) PushFront(val int) {
	f.left.PushFront(val)
	f.balance()
}

func (f *FrontMiddleBackQueue) PushMiddle(val int) {
	if f.left.Len() < f.right.Len() {
		f.left.PushBack(val)
		return
	}
	f.right.PushFront(val)
}

func (f *FrontMiddleBackQueue) PushBack(val int) {
	f.right.PushBack(val)
	f.balance()
}

func (f *FrontMiddleBackQueue) PopFront() int {
	if f.right.Len() == 0 {
		return -1
	}
	defer f.balance()
	res := 0
	if f.left.Len() > 0 {
		res = f.left.Remove(f.left.Front()).(int)
	} else {
		res = f.right.Remove(f.right.Front()).(int)
	}
	return res
}

func (f *FrontMiddleBackQueue) PopMiddle() int {
	if f.right.Len() == 0 {
		return -1
	}
	if f.left.Len() == f.right.Len() {
		return f.left.Remove(f.left.Back()).(int)
	}
	return f.right.Remove(f.right.Front()).(int)
}

func (f *FrontMiddleBackQueue) PopBack() int {
	if f.right.Len() == 0 {
		return -1
	}
	defer f.balance()
	return f.right.Remove(f.right.Back()).(int)
}

/**
 * Your FrontMiddleBackQueue object will be instantiated and called as such:
 * obj := Constructor();
 * obj.PushFront(val);
 * obj.PushMiddle(val);
 * obj.PushBack(val);
 * param_4 := obj.PopFront();
 * param_5 := obj.PopMiddle();
 * param_6 := obj.PopBack();
 */
