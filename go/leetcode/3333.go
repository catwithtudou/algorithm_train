package leetcode

func possibleStringCountII(word string, k int) int {
	if len(word) < k {
		return 0
	}

	const mod = 1_000_000_007

	cnts := []int{}
	ans, cnt := 1, 0

	for i := range word {
		cnt++
		if i == len(word)-1 || word[i] != word[i+1] {
			if cnt > 1 {
				if k > 0 {
					cnts = append(cnts, cnt-1)
				}
				ans = ans * cnt % mod
			}
			k--
			cnt = 0
		}
	}

	if k <= 0 {
		return ans
	}

	m := len(cnts)
	f := make([][]int, m+1)

	for i := range f {
		f[i] = make([]int, k)
	}
	for i := range f[0] {
		f[0][i] = 1
	}

	s := make([]int, k+1)

	for i, c := range cnts {
		for j, v := range f[i] {
			s[j+1] = s[j] + v
		}
		for j := range f[i+1] {
			f[i+1][j] = (s[j+1] - s[max(j-c, 0)]) % mod
		}
	}

	return (ans - f[m][k-1] + mod) % mod
}
