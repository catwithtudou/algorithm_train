package leetcode

func doesValidArrayExist(derived []int) bool {
	xor := 0

	for _, v := range derived {
		xor ^= v
	}

	return xor == 0
}
