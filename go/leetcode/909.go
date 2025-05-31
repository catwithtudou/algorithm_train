package leetcode

func snakesAndLadders(board [][]int) (step int) {
	n := len(board)
	vis := make([]bool, n*n+1)
	vis[1] = true
	q := []int{1}
	for len(q) > 0 {
		tmp := q
		q = nil
		for _, x := range tmp {
			if x == n*n {
				return
			}
			for y := x + 1; y <= min(x+6, n*n); y++ {
				r, c := (y-1)/n, (y-1)%n
				if r%2 > 0 {
					c = n - 1 - c
				}
				nxt := board[n-1-r][c]
				if nxt < 0 {
					nxt = y
				}
				if !vis[nxt] {
					vis[nxt] = true
					q = append(q, nxt)
				}
			}
		}
		step++
	}
	return -1
}
