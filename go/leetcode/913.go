package leetcode

func catMouseGame(graph [][]int) int {
	const (
		DRAW       = 0
		MOUSE_TURN = 1
		CAT_TURN   = 2
		MOUSE_WIN  = 1
		CAT_WIN    = 2
	)

	n := len(graph)
	color := make([][][]int, n)
	degree := make([][][]int, n)

	// 初始化color和degree数组
	for m := 0; m < n; m++ {
		color[m] = make([][]int, n)
		degree[m] = make([][]int, n)
		for c := 0; c < n; c++ {
			color[m][c] = make([]int, 3)
			degree[m][c] = make([]int, 3)
			// 鼠的回合，可移动的步数为graph[m]的长度
			degree[m][c][MOUSE_TURN] = len(graph[m])
			// 猫的回合，可移动的步数为graph[c]中非0的节点数
			catDegree := 0
			for _, neighbor := range graph[c] {
				if neighbor != 0 {
					catDegree++
				}
			}
			degree[m][c][CAT_TURN] = catDegree
		}
	}

	queue := [][4]int{} // 元素为 [m, c, t, result]

	// 处理初始状态：鼠在0或猫鼠同位置
	for m := 0; m < n; m++ {
		for c := 0; c < n; c++ {
			if m == 0 {
				// 鼠赢，无论当前是谁的回合
				color[m][c][MOUSE_TURN] = MOUSE_WIN
				color[m][c][CAT_TURN] = MOUSE_WIN
				queue = append(queue, [4]int{m, c, MOUSE_TURN, MOUSE_WIN})
				queue = append(queue, [4]int{m, c, CAT_TURN, MOUSE_WIN})
			} else if m == c {
				// 猫赢，无论当前是谁的回合
				color[m][c][MOUSE_TURN] = CAT_WIN
				color[m][c][CAT_TURN] = CAT_WIN
				queue = append(queue, [4]int{m, c, MOUSE_TURN, CAT_WIN})
				queue = append(queue, [4]int{m, c, CAT_TURN, CAT_WIN})
			}
		}
	}

	getPrevStates :=
		// 获取前驱状态
		func(graph [][]int, m, c, t int) [][3]int {
			var prevStates [][3]int
			if t == MOUSE_TURN {
				// 当前是鼠的回合，前一步是猫移动后的状态
				for _, prevC := range graph[c] {
					if prevC != 0 {
						prevStates = append(prevStates, [3]int{m, prevC, CAT_TURN})
					}
				}
			} else {
				// 当前是猫的回合，前一步是鼠移动后的状态
				for _, prevM := range graph[m] {
					prevStates = append(prevStates, [3]int{prevM, c, MOUSE_TURN})
				}
			}
			return prevStates
		}

	// BFS处理队列中的状态
	for len(queue) > 0 {
		curr := queue[0]
		queue = queue[1:]
		m, c, t, res := curr[0], curr[1], curr[2], curr[3]

		// 获取所有可能的前驱状态
		prevStates := getPrevStates(graph, m, c, t)
		for _, prev := range prevStates {
			prevM, prevC, prevT := prev[0], prev[1], prev[2]
			if color[prevM][prevC][prevT] != DRAW {
				continue // 已处理过
			}
			// 判断前驱状态是否可以确定结果
			if (prevT == MOUSE_TURN && res == MOUSE_WIN) || (prevT == CAT_TURN && res == CAT_WIN) {
				color[prevM][prevC][prevT] = res
				queue = append(queue, [4]int{prevM, prevC, prevT, res})
			} else {
				degree[prevM][prevC][prevT]--
				if degree[prevM][prevC][prevT] == 0 {
					// 所有移动都导致对方胜利
					loseRes := CAT_WIN
					if prevT == CAT_TURN {
						loseRes = MOUSE_WIN
					}
					color[prevM][prevC][prevT] = loseRes
					queue = append(queue, [4]int{prevM, prevC, prevT, loseRes})
				}
			}
		}
	}

	// 返回初始状态的结果
	return color[1][2][MOUSE_TURN]
}
