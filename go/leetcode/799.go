package leetcode

func champagneTower(poured, queryRow, queryGlass int) float64 {
	cur := []float64{float64(poured)}

	for i := 1; i <= queryRow; i++ {
		nxt := make([]float64, i+1)
		for j, x := range cur {
			if x > 1 {
				nxt[j] += (x - 1) / 2
				nxt[j+1] += (x - 1) / 2
			}
		}
		cur = nxt
	}

	return minFloat64(cur[queryGlass], 1)
}
