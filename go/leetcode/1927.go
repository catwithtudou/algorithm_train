package leetcode

func calSumGame(cal string) (q, sum int) {
	for _, x := range cal {
		if x == '?' {
			q++
		} else {
			sum += int(x - '0')
		}
	}
	return
}

func sumGame(num string) bool {
	n := len(num)
	ql, sumL := calSumGame(num[:n/2])
	qr, sumR := calSumGame(num[n/2:])
	return (ql+qr)%2 > 0 || (ql-qr)/2*9 != sumR-sumL
}
