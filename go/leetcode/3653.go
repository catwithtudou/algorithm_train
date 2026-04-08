package leetcode

func xorAfterQueries(nums []int, queries [][]int) int {
	const mod = 1e9 + 7

	for _, q := range queries {
		l, r, k, v := q[0], q[1], q[2], q[3]
		for i := l; i <= r; i += k {
			nums[i] = int((int64(nums[i]) * int64(v)) % mod)
		}
	}

	res := 0
	for _, v := range nums {
		res ^= v
	}

	return res
}
