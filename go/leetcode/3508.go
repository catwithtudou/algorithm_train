package leetcode

import "sort"

type Router struct {
	memoryLimit      int
	data             []RouterData
	dataSet          map[RouterData]struct{}
	destToTimestamps map[int][]int
}

type RouterData struct {
	source      int
	destination int
	timestamp   int
}

func RouterConstructor(memoryLimit int) Router {
	return Router{
		memoryLimit:      memoryLimit,
		data:             make([]RouterData, 0),
		dataSet:          make(map[RouterData]struct{}),
		destToTimestamps: make(map[int][]int),
	}
}

func (this *Router) AddPacket(source int, destination int, timestamp int) bool {
	data := RouterData{
		source:      source,
		destination: destination,
		timestamp:   timestamp,
	}

	if _, ok := this.dataSet[data]; ok {
		return false
	}

	this.dataSet[data] = struct{}{}

	if len(this.data) == this.memoryLimit {
		this.ForwardPacket()
	}

	this.data = append(this.data, data)
	this.destToTimestamps[destination] = append(this.destToTimestamps[destination], timestamp)

	return true
}

func (this *Router) ForwardPacket() []int {
	if len(this.data) == 0 {
		return []int{}
	}

	data := this.data[0]
	this.data = this.data[1:]
	this.destToTimestamps[data.destination] = this.destToTimestamps[data.destination][1:]
	delete(this.dataSet, data)

	return []int{data.source, data.destination, data.timestamp}
}

func (this *Router) GetCount(destination int, startTime int, endTime int) int {
	timestamps := this.destToTimestamps[destination]
	return sort.SearchInts(timestamps, endTime+1) - sort.SearchInts(timestamps, startTime)
}

/**
 * Your Router object will be instantiated and called as such:
 * obj := Constructor(memoryLimit);
 * param_1 := obj.AddPacket(source,destination,timestamp);
 * param_2 := obj.ForwardPacket();
 * param_3 := obj.GetCount(destination,startTime,endTime);
 */
