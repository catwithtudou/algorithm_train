package leetcode

func numBusesToDestination(routes [][]int, source int, target int) int {
	busStop := make(map[int][]int, 0)
	for i, route := range routes {
		for _, x := range route {
			busStop[x] = append(busStop[x], i)
		}
	}

	if busStop[source] == nil || busStop[target] == nil {
		if source != target {
			return -1
		}
		return 0
	}

	dis := map[int]int{source: 0}
	q := []int{source}
	for len(q) > 0 {
		x := q[0]
		q = q[1:]
		disX := dis[x]
		for _, i := range busStop[x] {
			for _, y := range routes[i] {
				if _, ok := dis[y]; !ok {
					dis[y] = disX + 1
					q = append(q, y)
				}
			}
			routes[i] = nil
		}
	}

	if d, ok := dis[target]; ok {
		return d
	}

	return -1
}
