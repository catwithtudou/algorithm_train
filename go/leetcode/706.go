package leetcode

import "container/list"

type MyHashMap struct {
	data []list.List
	base int
}

type kvEntry struct {
	key, value int
}

func ConstructorMyHashMap() MyHashMap {
	return MyHashMap{make([]list.List, 769), 769}
}

func (this *MyHashMap) hash(key int) int {
	return key % this.base
}

func (this *MyHashMap) Put(key int, value int) {
	h := this.hash(key)
	for i := this.data[h].Front(); i != nil; i = i.Next() {
		if i.Value.(kvEntry).key == key {
			i.Value = kvEntry{key, value}
			return
		}
	}
	this.data[h].PushBack(kvEntry{key, value})
}

func (this *MyHashMap) Get(key int) int {
	h := this.hash(key)
	for i := this.data[h].Front(); i != nil; i = i.Next() {
		if et := i.Value.(kvEntry); et.key == key {
			return et.value
		}
	}
	return -1
}

func (this *MyHashMap) Remove(key int) {
	h := this.hash(key)
	for e := this.data[h].Front(); e != nil; e = e.Next() {
		if e.Value.(kvEntry).key == key {
			this.data[h].Remove(e)
		}
	}

}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Put(key,value);
 * param_2 := obj.Get(key);
 * obj.Remove(key);
 */
