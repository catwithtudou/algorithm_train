package leetcode

func sumCounts2916(nums []int) int {
	const mod = 1_000_000_007
	n := len(nums)
	t1 := make([]int, n+1) // 差分 d[i]
	t2 := make([]int, n+1) // i * d[i]
	upd := func(i, v int) {
		w := i * v
		for ; i <= n; i += i & -i {
			t1[i] += v
			t2[i] += w
		}
	}
	pre := func(p int) int { // f[1..p] 的和
		s1, s2 := 0, 0
		for i := p; i > 0; i -= i & -i {
			s1 += t1[i]
			s2 += t2[i]
		}
		return (p+1)*s1 - s2
	}

	last := make([]int, 100001) // 值 -> 上次出现位置，0 表示没出现
	ans, s := 0, 0
	for r := 1; r <= n; r++ {
		x := nums[r-1]
		j := last[x]
		s = (s + 2*(pre(r)-pre(j)) + (r - j)) % mod // 平方和增量
		upd(j+1, 1)
		upd(r+1, -1) // f[j+1..r] 都 +1
		ans = (ans + s) % mod
		last[x] = r
	}
	return ans
}
