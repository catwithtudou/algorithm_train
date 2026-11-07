package leetcode

import "container/heap"

func assignTasks(servers []int, tasks []int) []int {
	idle := make(hpTask, len(servers))
	for i, w := range servers {
		idle[i] = pairTask{w, i}
	}
	heap.Init(&idle)

	busy := hpTask{}
	now := 0

	release := func() {
		for len(busy) > 0 && busy[0].t <= now {
			i := heap.Pop(&busy).(pairTask).i
			heap.Push(&idle, pairTask{servers[i], i})
		}
	}

	for i, t := range tasks {
		now = max(now, i)

		release()

		if len(idle) == 0 {
			now = busy[0].t
			release()
		}

		j := heap.Pop(&idle).(pairTask).i
		heap.Push(&busy, pairTask{now + t, j})
		tasks[i] = j
	}

	return tasks
}

type pairTask struct{ t, i int }
type hpTask []pairTask

func (h hpTask) Len() int           { return len(h) }
func (h hpTask) Less(i, j int) bool { a, b := h[i], h[j]; return a.t < b.t || a.t == b.t && a.i < b.i }
func (h hpTask) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *hpTask) Push(v any)        { *h = append(*h, v.(pairTask)) }
func (h *hpTask) Pop() any          { a := *h; v := a[len(a)-1]; *h = a[:len(a)-1]; return v }
