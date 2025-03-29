package leetcode

func longestCycle(edges []int) int {
	n := len(edges)
	indegree := make([]int, n)

	for _, edge := range edges {
		if edge != -1 {
			indegree[edge]++
		}
	}

	queue := []int{}
	for i := 0; i < n; i++ {
		if indegree[i] == 0 {
			queue = append(queue, i)
		}
	}
	for len(queue) > 0 {
		node := queue[0]
		queue = queue[1:]

		nextNode := edges[node]
		if nextNode != -1 {
			indegree[nextNode]--
			if indegree[nextNode] == 0 {
				queue = append(queue, nextNode)
			}
		}
	}

	visited := make([]bool, n)
	ans := -1

	for i := 0; i < n; i++ {
		if indegree[i] > 0 && !visited[i] {
			cycleLen := findCycleLength(i, edges, &visited)
			ans = max(ans, cycleLen)
		}
	}

	return ans
}

func findCycleLength(start int, edges []int, visited *[]bool) int {
	dist := make(map[int]int)
	dist[start] = 0

	curr := start
	for {
		(*visited)[curr] = true
		next := edges[curr]

		if next == -1 {
			return -1
		}

		if pos, ok := dist[next]; ok {
			return dist[curr] - pos + 1
		}

		if (*visited)[next] {
			return -1
		}

		dist[next] = dist[curr] + 1
		curr = next
	}
}
