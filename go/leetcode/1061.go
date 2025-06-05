package leetcode

func smallestEquivalentString(s1 string, s2 string, baseStr string) string {
	fa := [26]byte{}
	for i := range fa {
		fa[i] = byte(i)
	}

	var find func(byte) byte
	find = func(x byte) byte {
		if fa[x] != x {
			fa[x] = find(fa[x])
		}
		return fa[x]
	}

	merge := func(x, y byte) {
		small, big := find(x), find(y)
		if small > big {
			small, big = big, small
		}
		fa[big] = small
	}

	for i, x := range s1 {
		merge(byte(x)-'a', s2[i]-'a')
	}

	result := make([]byte, len(baseStr))
	for i, c := range baseStr {
		result[i] = find(byte(c)-'a') + 'a'
	}

	return string(result)
}
