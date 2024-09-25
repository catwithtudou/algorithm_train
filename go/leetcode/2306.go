package leetcode

func distinctNames(ideas []string) (ans int64) {
	group := [26]map[string]bool{}
	for i := range group {
		group[i] = make(map[string]bool, 0)
	}

	for _, s := range ideas {
		group[s[0]-'a'][s[1:]] = true
	}

	for i, a := range group {
		for _, b := range group[:i] {
			m := 0
			for s := range a {
				if b[s] {
					m++
				}
			}
			ans += int64(len(a)-m) * int64(len(b)-m)
		}
	}

	return ans * 2
}
