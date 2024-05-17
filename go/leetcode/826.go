package leetcode

import "golang.org/x/exp/slices"

func maxProfitAssignment(difficulty []int, profit []int, worker []int) int {
	n := len(difficulty)
	type job struct {
		dif int
		pro int
	}
	jobs := make([]job, 0, n)
	for i, d := range difficulty {
		jobs = append(jobs, job{d, profit[i]})
	}
	slices.SortFunc(jobs, func(a, b job) int {
		return a.dif - b.dif
	})
	slices.Sort(worker)
	ans := 0
	j, maxProfit := 0, 0
	for _, i := range worker {
		for j < n && jobs[j].dif <= i {
			maxProfit = max(maxProfit, jobs[j].pro)
			j++
		}
		ans += maxProfit
	}

	return ans
}
