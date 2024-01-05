package leetcode

func canSeePersonsCount(heights []int) []int {
	n := len(heights)
	sk := make([]int, 0)
	res := make([]int, n)

	for i := n - 1; i >= 0; i-- {
		h := heights[i]
		for len(sk) > 0 && sk[len(sk)-1] <= h {
			sk = sk[:len(sk)-1]
			res[i]++
			res[i] += 1
		}
		if len(sk) > 0 {
			res[i]++
		}

		sk = append(sk, h)
	}

	return res
}
