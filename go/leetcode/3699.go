package leetcode

func zigZagArrays(n int, l int, r int) int {
	k := r - l + 1
	f0 := make([]int, k)
	f1 := make([]int, k)
	for i := range f0 {
		f0[i] = 1
		f1[i] = 1
	}

	s0 := make([]int, k+1)
	s1 := make([]int, k+1)
	for range n - 1 {
		for i, v := range f0 {
			s0[i+1] = s0[i] + v
			s1[i+1] = s1[i] + f1[i]
		}
		for i := range f0 {
			f0[i] = s1[i] % mod
			f1[i] = (s0[k] - s0[i+1]) % mod
		}
	}

	ans := 0
	for i, v := range f0 {
		ans += v + f1[i]
	}

	return ans % mod
}
