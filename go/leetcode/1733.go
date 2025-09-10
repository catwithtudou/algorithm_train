package leetcode

func minimumTeachings(n int, languages [][]int, friendships [][]int) int {
	m := len(languages)
	learned := make([][]bool, m)
	for i, list := range languages {
		learned[i] = make([]bool, n+1)
		for _, lang := range list {
			learned[i][lang] = true
		}
	}

	todoList := [][2]int{}

next:
	for _, f := range friendships {
		u, v := f[0]-1, f[1]-1
		for _, x := range languages[u] {
			if learned[v][x] {
				continue next
			}
		}
		todoList = append(todoList, [2]int{u, v})
	}

	ans := m
	for k := 1; k <= n; k++ {
		set := map[int]struct{}{}
		for _, f := range todoList {
			u, v := f[0], f[1]
			if !learned[u][k] {
				set[u] = struct{}{}
			}
			if !learned[v][k] {
				set[v] = struct{}{}
			}
		}
		ans = min(ans, len(set))
	}

	return ans
}
