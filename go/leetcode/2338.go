package leetcode

const maxN = 10_000
const maxE = 13

var exp [maxN + 1][]int
var c [maxN + maxE][maxE + 1]int

func init() {
	for x := 2; x <= maxN; x++ {
		t := x
		for i := 2; i*i <= t; i++ {
			e := 0
			for ; t%i == 0; t /= i {
				e++
			}
			if e > 0 {
				exp[x] = append(exp[x], e)
			}
		}
		if t > 1 {
			exp[x] = append(exp[x], 1)
		}

	}

	for i := range c {
		c[i][0] = 1
		for j := 1; j <= min(i, maxE); j++ {
			c[i][j] = (c[i-1][j] + c[i-1][j-1]) % mod
		}
	}

}

func idealArrays(n int, maxValue int) (ans int) {
	for x := 1; x <= maxValue; x++ {
		res := 1
		for _, e := range exp[x] {
			res = res * c[e+n-1][e] % mod
		}
		ans += res
	}
	return ans % mod
}
