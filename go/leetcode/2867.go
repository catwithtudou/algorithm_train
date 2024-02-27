package leetcode

// 埃氏筛
const Mx = 100001

var is_prime [Mx]bool

func init() {
	for i := 0; i < Mx; i++ {
		is_prime[i] = true
	}
	is_prime[1] = false
	for i := 2; i*i < Mx; i++ {
		if is_prime[i] {
			for j := i * i; j < Mx; j += i {
				is_prime[j] = false
			}
		}
	}
}

func countPaths(n int, edges [][]int) int64 {
	G := make([][]int, n+1)
	for _, edge := range edges {
		i, j := edge[0], edge[1]
		G[i] = append(G[i], j)
		G[j] = append(G[j], i)
	}

	var dfs func(int, int)
	var seen []int
	dfs = func(i, pre int) {
		seen = append(seen, i)
		for _, j := range G[i] {
			if j != pre && !is_prime[j] {
				dfs(j, i)
			}
		}
	}
	res := int64(0)
	count := make([]int64, n+1)
	for i := 1; i <= n; i++ {
		if !is_prime[i] {
			continue
		}
		cur := int64(0)
		for _, j := range G[i] {
			if is_prime[j] {
				continue
			}
			if count[j] == 0 {
				seen = []int{}
				dfs(j, 0)
				cnt := int64(len(seen))
				for _, k := range seen {
					count[k] = cnt
				}
			}
			res += count[j] * cur
			cur += count[j]
		}
		res += cur
	}
	return res
}
