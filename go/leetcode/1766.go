package leetcode

const mx = 51

var coprime [mx][]int

func newCoprime() {
	for i := 1; i < mx; i++ {
		for j := 1; j < mx; j++ {
			if gcd(i, j) == 1 {
				coprime[i] = append(coprime[i], j)
			}
		}
	}
}

func getCoprimes(nums []int, edges [][]int) []int {
	newCoprime()

	n := len(nums)
	g := make([][]int, n)
	for _, e := range edges {
		x, y := e[0], e[1]
		g[x] = append(g[x], y)
		g[y] = append(g[y], x)
	}

	ans := make([]int, n)
	for i := range ans {
		ans[i] = -1
	}
	type pair struct{ depth, id int }
	valDepthId := [mx]pair{}
	var dfs func(int, int, int)
	dfs = func(x, fa, depth int) {
		val := nums[x]
		maxDepth := 0
		for _, j := range coprime[val] {
			p := valDepthId[j]
			if p.depth > maxDepth {
				maxDepth = p.depth
				ans[x] = p.id
			}
		}

		tmp := valDepthId[val]
		valDepthId[val] = pair{depth, x}
		for _, y := range g[x] {
			if y != fa {
				dfs(y, x, depth+1)
			}
		}
		valDepthId[val] = tmp
	}
	dfs(0, -1, 1)
	return ans
}
