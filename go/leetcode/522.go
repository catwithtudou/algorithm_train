package leetcode

func isSubDep(s, t string) bool {
	i := 0
	for _, c := range t {
		if s[i] == byte(c) {
			i++
			if len(s) == i {
				return true
			}
		}
	}
	return false
}

func findLUSlength522(strs []string) int {
	ans := -1
next:
	for i, s := range strs {
		if len(s) <= ans {
			continue
		}
		for j, t := range strs {
			if j != i && isSubDep(s, t) {
				continue next
			}
		}
		ans = len(s)
	}

	return ans
}
