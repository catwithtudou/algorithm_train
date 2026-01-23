package leetcode

import (
	"cmp"

	"github.com/emirpasic/gods/v2/trees/redblacktree"
)

func minimumPairRemovalII(nums []int) (ans int) {
	n := len(nums)
	type pair struct{ s, i int }
	// (相邻元素和，左边那个数的下标)
	pairs := redblacktree.NewWith[pair, struct{}](func(a, b pair) int { return cmp.Or(a.s-b.s, a.i-b.i) })
	dec := 0 // 递减的相邻对的个数
	for i := range n - 1 {
		x, y := nums[i], nums[i+1]
		if x > y {
			dec++
		}
		pairs.Put(pair{x + y, i}, struct{}{})
	}

	// 剩余下标
	idx := redblacktree.New[int, struct{}]()
	for i := range n {
		idx.Put(i, struct{}{})
	}

	for dec > 0 {
		ans++

		it := pairs.Left()
		s := it.Key.s
		i := it.Key.i
		pairs.Remove(it.Key) // 删除相邻元素和最小的一对

		// (当前元素，下一个数)
		node, _ := idx.Ceiling(i + 1)
		nxt := node.Key
		if nums[i] > nums[nxt] { // 旧数据
			dec--
		}

		// (前一个数，当前元素)
		node, _ = idx.Floor(i - 1)
		if node != nil {
			pre := node.Key
			if nums[pre] > nums[i] { // 旧数据
				dec--
			}
			if nums[pre] > s { // 新数据
				dec++
			}
			pairs.Remove(pair{nums[pre] + nums[i], pre})
			pairs.Put(pair{nums[pre] + s, pre}, struct{}{})
		}

		// (下一个数，下下一个数)
		node, _ = idx.Ceiling(nxt + 1)
		if node != nil {
			nxt2 := node.Key
			if nums[nxt] > nums[nxt2] { // 旧数据
				dec--
			}
			if s > nums[nxt2] { // 新数据（当前元素，下下一个数）
				dec++
			}
			pairs.Remove(pair{nums[nxt] + nums[nxt2], nxt})
			pairs.Put(pair{s + nums[nxt2], i}, struct{}{})
		}

		nums[i] = s     // 把 nums[nxt] 加到 nums[i] 中
		idx.Remove(nxt) // 删除 nxt
	}
	return
}
