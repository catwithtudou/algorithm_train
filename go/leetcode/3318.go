package leetcode

import "sort"

func findXSum(nums []int, k int, x int) []int {
	n := len(nums)
	if k == 0 || k > n {
		return []int{}
	}

	res := make([]int, n-k+1)
	cnt := make(map[int]int)

	for i := 0; i < k; i++ {
		cnt[nums[i]]++
	}
	res[0] = xSumByCnt(cnt, x)

	for i := k; i < n; i++ {
		leave := nums[i-k]
		cnt[leave]--
		if cnt[leave] == 0 {
			delete(cnt, leave)
		}
		cnt[nums[i]]++
		res[i-k+1] = xSumByCnt(cnt, x)
	}

	return res
}

func xSumByCnt(cnt map[int]int, x int) int {
	type pair struct {
		val int
		f   int
	}

	ps := make([]pair, 0, len(cnt))
	for v, f := range cnt {
		ps = append(ps, pair{v, f})
	}
	sort.Slice(ps, func(i, j int) bool {
		if ps[i].f != ps[j].f {
			return ps[i].f > ps[j].f
		}
		return ps[i].val > ps[j].val
	})

	sum, take := 0, 0
	for _, p := range ps {
		if take == x {
			break
		}
		sum += p.val * p.f
		take++
	}

	return sum
}
