package leetcode

func catMouseGameII(gMouse, gCat [][]int, mouseStart, catStart, hole int) int {
	n := len(gMouse)
	deg := make([][][2]int, n)
	for i := range deg {
		deg[i] = make([][2]int, n)
	}
	for i, m := range gMouse {
		for j, c := range gCat {
			deg[i][j][0] = len(m)
			deg[i][j][1] = len(c)
		}
	}

	winner := make([][][2]int, n)
	for i := range winner {
		winner[i] = make([][2]int, n)
	}
	q := [][3]int{}
	for i := range n {
		winner[hole][i][1] = 1 // 鼠到达洞中（此时轮到猫移动），鼠获胜
		winner[i][hole][0] = 2 // 猫到达洞中（此时轮到鼠移动），猫获胜
		winner[i][i][0] = 2    // 猫和鼠出现在同一个节点，无论轮到谁移动，都是猫获胜
		winner[i][i][1] = 2
		q = append(q, [3]int{hole, i, 1})
		q = append(q, [3]int{i, hole, 0})
		q = append(q, [3]int{i, i, 0})
		q = append(q, [3]int{i, i, 1})
	}

	// 获取 (mouse, cat, turn) 的上个状态（值尚未确定）
	getPreStates := func(mouse, cat, turn int) (preStates [][2]int) {
		if turn == 0 { // 当前轮到鼠移动
			for _, preCat := range gCat[cat] { // 上一轮猫的位置
				if winner[mouse][preCat][1] == 0 { // 猫无法移动到洞中
					preStates = append(preStates, [2]int{mouse, preCat})
				}
			}
		} else { // 当前轮到猫移动
			for _, preMouse := range gMouse[mouse] { // 上一轮鼠的位置
				if winner[preMouse][cat][0] == 0 {
					preStates = append(preStates, [2]int{preMouse, cat})
				}
			}
		}
		return preStates
	}

	// 减少上个状态的度数
	decDegToZero := func(preMouse, preCat, preTurn int) bool {
		deg[preMouse][preCat][preTurn]--
		return deg[preMouse][preCat][preTurn] == 0
	}

	for len(q) > 0 {
		mouse, cat, turn := q[0][0], q[0][1], q[0][2]
		q = q[1:]
		win := winner[mouse][cat][turn] // 最终谁赢了
		for _, pre := range getPreStates(mouse, cat, turn) {
			preMouse, preCat, preTurn := pre[0], pre[1], turn^1
			if preTurn == win-1 || decDegToZero(preMouse, preCat, preTurn) {
				winner[preMouse][preCat][preTurn] = win
				q = append(q, [3]int{preMouse, preCat, preTurn}) // 继续倒推
			}
		}
	}

	return winner[mouseStart][catStart][0]
}

func canMouseWin(grid []string, catJump, mouseJump int) bool {
	dirs := [4]struct{ dx, dy int }{{0, -1}, {0, 1}, {-1, 0}, {1, 0}} // 左右上下
	m, n := len(grid), len(grid[0])
	// 鼠和猫分别建图
	gMouse := make([][]int, m*n)
	gCat := make([][]int, m*n)
	var mx, my, cx, cy, fx, fy int
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if grid[i][j] == '#' { // 墙
				continue
			}
			if grid[i][j] == 'M' { // 鼠的位置
				mx, my = i, j
			} else if grid[i][j] == 'C' { // 猫的位置
				cx, cy = i, j
			} else if grid[i][j] == 'F' { // 食物（洞）的位置
				fx, fy = i, j
			}
			v := i*n + j             // 二维坐标 (i,j) 映射为一维坐标 v
			for _, d := range dirs { // 枚举左右上下四个方向
				for k := range mouseJump + 1 { // 枚举跳跃长度
					x, y := i+k*d.dx, j+k*d.dy
					if x < 0 || x >= m || y < 0 || y >= n || grid[x][y] == '#' { // 出界或者遇到墙
						break
					}
					gMouse[v] = append(gMouse[v], x*n+y) // 连边
				}
				for k := range catJump + 1 { // 枚举跳跃长度
					x, y := i+k*d.dx, j+k*d.dy
					if x < 0 || x >= m || y < 0 || y >= n || grid[x][y] == '#' { // 出界或者遇到墙
						break
					}
					gCat[v] = append(gCat[v], x*n+y) // 连边
				}
			}
		}
	}

	return catMouseGameII(gMouse, gCat, mx*n+my, cx*n+cy, fx*n+fy) == 1
}
