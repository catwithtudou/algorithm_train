package leetcode

func minJumps(arr []int) (ans int) {
	pos := make(map[int][]int)
	for i, x := range arr {
		pos[x] = append(pos[x], i)
	}

	n := len(arr)
	vis := make([]bool, n)
	vis[0] = true
	q := []int{0}

	for ; ; ans++ {
		tmp := q
		q = nil

		for _, i := range tmp {
			if i == n-1 {
				return
			}

			if !vis[i+1] {
				vis[i+1] = true
				q = append(q, i+1)
			}

			if i > 0 && !vis[i-1] {
				vis[i-1] = true
				q = append(q, i-1)
			}

			x := arr[i]
			if pos[x] == nil {
				continue
			}

			for _, j := range pos[x] {
				if !vis[j] {
					vis[j] = true
					q = append(q, j)
				}
			}

			delete(pos, x)
		}
	}

	return
}
