package leetcode

import "container/heap"

func eatenApples(apples []int, days []int) (ans int) {
	hp := appleHp{}
	for i, num := range apples {
		for len(hp) > 0 && hp[0].day == i {
			heap.Pop(&hp)
		}
		if num > 0 {
			heap.Push(&hp, applePair{num, i + days[i]})
		}
		if len(hp) > 0 {
			ans++
			hp[0].num--
			if hp[0].num == 0 {
				heap.Pop(&hp)
			}
		}
	}

	i := len(apples)
	for {
		for len(hp) > 0 && hp[0].day <= i {
			heap.Pop(&hp)
		}
		if len(hp) == 0 || hp[0].num == 0 {
			return
		}
		k := min(hp[0].num, hp[0].day-i)
		heap.Pop(&hp)
		ans += k
		i += k
	}
}

type applePair struct{ num, day int }
type appleHp []applePair

func (h appleHp) Len() int            { return len(h) }
func (h appleHp) Less(i, j int) bool  { return h[i].day < h[j].day }
func (h appleHp) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *appleHp) Push(v interface{}) { *h = append(*h, v.(applePair)) }
func (h *appleHp) Pop() interface{}   { a := *h; v := a[len(a)-1]; *h = a[:len(a)-1]; return v }
