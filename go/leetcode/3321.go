package leetcode

import (
	"cmp"

	"github.com/emirpasic/gods/v2/trees/redblacktree"
)

func findXSumII(nums []int, k int, x int) []int64 {
	L := redblacktree.NewWith[numCnt, struct{}](lessNum)
	R := redblacktree.NewWith[numCnt, struct{}](lessNum)

	sumL := 0
	cnt := map[int]int{}
	add := func(x int) {
		p := numCnt{cnt[x], x}
		if p.c == 0 {
			return
		}
		if !L.Empty() && lessNum(p, L.Left().Key) > 0 {
			sumL += p.c * p.x
			L.Put(p, struct{}{})
		} else {
			R.Put(p, struct{}{})
		}
	}
	del := func(x int) {
		p := numCnt{cnt[x], x}
		if p.c == 0 {
			return
		}
		if _, ok := L.Get(p); ok {
			sumL -= p.c * p.x
			L.Remove(p)
		} else {
			R.Remove(p)
		}
	}
	l2r := func() {
		p := L.Left().Key
		sumL -= p.c * p.x
		L.Remove(p)
		R.Put(p, struct{}{})
	}
	r2l := func() {
		p := R.Right().Key
		sumL += p.c * p.x
		R.Remove(p)
		L.Put(p, struct{}{})
	}

	ans := make([]int64, len(nums)-k+1)
	for r, in := range nums {
		// 添加 in
		del(in)
		cnt[in]++
		add(in)

		l := r + 1 - k
		if l < 0 {
			continue
		}

		for !R.Empty() && L.Size() < x {
			r2l()
		}
		for L.Size() > x {
			l2r()
		}
		ans[l] = int64(sumL)

		out := nums[l]
		del(out)
		cnt[out]--
		add(out)
	}
	return ans

}

type numCnt struct{ c, x int }

func lessNum(p, q numCnt) int {
	return cmp.Or(p.c-q.c, p.x-q.x)
}
