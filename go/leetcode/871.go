package leetcode

import (
	"container/heap"
	"sort"
)

func minRefuelStops(target int, startFuel int, stations [][]int) (ans int) {
	stations = append(stations, []int{target, 0})
	prePos, curFuel := 0, startFuel
	fuelHeap := &refuelHp{}
	for _, station := range stations {
		pos, fuel := station[0], station[1]
		curFuel -= pos - prePos
		for fuelHeap.Len() > 0 && curFuel < 0 {
			curFuel += heap.Pop(fuelHeap).(int)
			ans++
		}
		if curFuel < 0 {
			return -1
		}
		heap.Push(fuelHeap, fuel)
		prePos = pos
	}
	return
}

type refuelHp struct{ sort.IntSlice }

func (h refuelHp) Less(i, j int) bool { return h.IntSlice[i] > h.IntSlice[j] }
func (h *refuelHp) Push(v any)        { h.IntSlice = append(h.IntSlice, v.(int)) }
func (h *refuelHp) Pop() any          { a := h.IntSlice; v := a[len(a)-1]; h.IntSlice = a[:len(a)-1]; return v }
