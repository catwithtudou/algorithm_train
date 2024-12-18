package leetcode

func calcZ(s string) []int {
	n := len(s)
	z := make([]int, n)
	z[0] = n
	l, r := 0, 0
	for i := 1; i < n; i++ {
		if i <= r {
			z[i] = min(z[i-l], r-i+1)
		}
		for i+z[i] < n && s[z[i]] == s[i+z[i]] {
			z[i]++
			l = i
			r = i + z[i] - 1
		}
	}
	return z
}

func jump(maxjumps []int) int {
	ans := 0
	curR, nxtR := 0, 0
	for i, v := range maxjumps {
		nxtR = max(nxtR, i+v)
		if i == curR {
			if i == nxtR {
				return -1
			}
			curR = nxtR
			ans++
		}
	}
	return ans
}

func minValidStringsII(words []string, target string) int {
	maxJumps := make([]int, len(target))
	for _, w := range words {
		z := calcZ(w + "#" + target)
		for i, z := range z[len(w)+1:] {
			maxJumps[i] = max(maxJumps[i], z)
		}
	}

	return jump(maxJumps)
}
