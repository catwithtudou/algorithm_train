package leetcode

import "container/heap"

func maxAverageRatio(classes [][]int, extraStudents int) float64 {

	n := len(classes)
	h := make(ratioHp, n)

	for i, c := range classes {
		a, b := c[0], c[1]
		h[i] = ratioTuple{float64(b-a) / float64(b*(b+1)), a, b}
	}
	heap.Init(&h)

	for range extraStudents {
		h[0].a++
		h[0].b++
		a, b := h[0].a, h[0].b
		h[0].gain = float64(b-a) / float64(b*(b+1))
		heap.Fix(&h, 0)
	}

	sum := 0.0

	for _, t := range h {
		sum += float64(t.a) / float64(t.b)
	}
	return sum / float64(n)
}

type ratioTuple struct {
	gain float64
	a, b int
}

type ratioHp []ratioTuple

func (h ratioHp) Len() int           { return len(h) }
func (h ratioHp) Less(i, j int) bool { return h[i].gain > h[j].gain }
func (h ratioHp) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h ratioHp) Push(any)           {}
func (h ratioHp) Pop() (_ any)       { return }
