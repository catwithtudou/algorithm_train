package leetcode

import "container/list"

type MyHashSet struct {
	data []list.List
}

func ConstructorMyHashSet() MyHashSet {
	return MyHashSet{make([]list.List, 769)}
}

func (this *MyHashSet) hash(key int) int {
	return key % 769
}

func (this *MyHashSet) Add(key int) {
	if this.Contains(key) {
		return
	}
	this.data[this.hash(key)].PushBack(key)
}

func (this *MyHashSet) Remove(key int) {
	realKey := this.hash(key)
	for e := this.data[realKey].Front(); e != nil; e = e.Next() {
		if e.Value.(int) == key {
			this.data[realKey].Remove(e)
		}
	}
}

func (this *MyHashSet) Contains(key int) bool {
	realKey := this.hash(key)
	for e := this.data[realKey].Front(); e != nil; e = e.Next() {
		if e.Value.(int) == key {
			return true
		}
	}
	return false
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Add(key);
 * obj.Remove(key);
 * param_3 := obj.Contains(key);
 */
