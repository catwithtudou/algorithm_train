package leetcode

func processStrII(s string, k int64) byte {
	n := len(s)
	size := make([]int64, n)
	sz := int64(0)
	for i, c := range s {
		if c == '*' {
			sz = maxInt64(sz-1, 0)
		} else if c == '#' {
			sz *= 2
		} else if c != '%' {
			sz++
		}
		size[i] = sz
	}

	if k >= size[n-1] {
		return '.'
	}

	for i := n - 1; ; i-- {
		c := s[i]
		sz = size[i]
		if c == '#' {
			if k >= sz/2 {
				k -= sz / 2
			}
		} else if c == '%' {
			k = sz - 1 - k
		} else if c != '*' && k == sz-1 {
			return c
		}
	}
}
