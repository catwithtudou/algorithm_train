package leetcode

func maxNumberOfFamilies(n int, reservedSeats [][]int) (ans int) {
	re := make(map[int]int)
	for _, x := range reservedSeats {
		seat := x[1]
		if seat >= 2 && seat <= 9 {
			re[x[0]] |= 1 << (seat - 2)
		}
	}

	empty := n - len(re)
	ans += empty * 2
	for _, x := range re {
		if x&0b1111 == 0 || x&0b111100 == 0 || x&0b11110000 == 0 {
			ans++
		}
	}
	return
}
