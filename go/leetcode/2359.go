package leetcode

func closestMeetingNode(edges []int, node1 int, node2 int) int {
	n := len(edges)
	calcDis := func(x int) []int {
		dis := make([]int, n)
		for i := range dis {
			dis[i] = n
		}
		for d := 0; x >= 0 && dis[x] == n; x = edges[x] {
			dis[x] = d
			d++
		}
		return dis
	}

	dis1 := calcDis(node1)
	dis2 := calcDis(node2)

	minDis, ans := n, -1
	for i, d1 := range dis1 {
		d := max(d1, dis2[i])
		if d < minDis {
			minDis, ans = d, i
		}
	}
	return ans
}
