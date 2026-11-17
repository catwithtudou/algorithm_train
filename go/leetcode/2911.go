package leetcode

func minimumChanges(s string, k int) int {
	n := len(s)
	modify := make([][]int, n-1)
	for l := range modify {
		modify[l] = make([]int, n)
		for r := l + 1; r < n; r++ {
			modify[l][r] = minimunInitCal(s[l : r+1])
		}
	}

	memo := make([][]int, k)
	for i := range memo {
		memo[i] = make([]int, n)
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}

	var dfs func(int, int) int

	dfs = func(i, j int) int {
		if i == 0 {
			return modify[0][j]
		}

		p := &memo[i][j]
		if *p != -1 {
			return *p
		}

		res := n
		for L := i * 2; L < j; L++ {
			res = min(res, dfs(i-1, L-1)+modify[L][j])
		}
		*p = res
		return res
	}

	return dfs(k-1, n-1)
}

const minimunMx = 200

var minimunDivisors [minimunMx + 1][]int

func minimunInit() {
	for i := 1; i <= minimunMx; i++ {
		for j := i * 2; j <= minimunMx; j += i {
			minimunDivisors[j] = append(minimunDivisors[j], i)
		}
	}
}

func minimunInitCal(s string) int {
	n := len(s)
	res := n
	for _, d := range minimunDivisors[n] {
		cnt := 0
		for i0 := 0; i0 < d; i0++ {
			for i, j := i0, n-d+i0; i < j; i, j = i+d, j-d {
				if s[i] != s[j] {
					cnt++
				}
			}
		}
		res = min(res, cnt)
	}
	return res
}
