package leetcode

func minimumCost3789(cost1 int, cost2 int, costBoth int, need1 int, need2 int) int64 {
	res1 := cost1*need1 + cost2*need2
	if need1 > need2 {
		need1, need2 = need2, need1
		cost2 = cost1
	}
	res2 := costBoth * need2
	res3 := costBoth*need1 + cost2*(need2-need1)
	return int64(min(res1, min(res2, res3)))
}
