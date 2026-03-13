package leetcode

import "container/heap"

func minNumberOfSeconds(mountainHeight int, workerTimes []int) int64 {
	h := make(hpWorker, len(workerTimes))
	for i, t := range workerTimes {
		h[i] = worker{total: t, cur: t, base: t}
	}
	heap.Init(&h)

	ans := 0

	for range mountainHeight {
		ans = h[0].total
		h[0].cur += h[0].base
		h[0].total += h[0].cur
		heap.Fix(&h, 0)
	}

	return int64(ans)
}

type worker struct{ total, cur, base int }
type hpWorker []worker

func (h hpWorker) Len() int           { return len(h) }
func (h hpWorker) Less(i, j int) bool { return h[i].total < h[j].total }
func (h hpWorker) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (hpWorker) Push(any)             {}
func (hpWorker) Pop() (_ any)         { return }
