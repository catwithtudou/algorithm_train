package leetcode

func productQueries(n int, queries [][]int) []int {

	const mod = 1_000_000_007

	powers := []int{}

	for n > 0 {
		lowbit := n & -n
		powers = append(powers, lowbit)
		n ^= lowbit
	}

	ans := make([]int, len(queries))

	for i, q := range queries {
		mul := 1
		for _, x := range powers[q[0] : q[1]+1] {
			mul = mul * x % mod
		}
		ans[i] = mul
	}

	return ans
}
