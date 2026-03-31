package leetcode

import "bytes"

func generateString(str1 string, str2 string) string {
	n, m := len(str1), len(str2)
	t := []byte(str2)
	ans := bytes.Repeat([]byte{'?'}, n+m-1)

	for i, s := range str1 {
		if s != 'T' {
			continue
		}

		sub := ans[i : i+m]
		for j, c := range sub {
			if c != '?' && c != t[j] {
				return ""
			}
			sub[j] = t[j]
		}
	}

	oldAns := ans
	ans = bytes.ReplaceAll(ans, []byte{'?'}, []byte{'a'})

next:
	for i, b := range str1 {
		if b != 'F' {
			continue
		}

		sub := ans[i : i+m]
		if !bytes.Equal(sub, t) {
			continue
		}

		old := oldAns[i : i+m]
		for j := m - 1; j >= 0; j-- {
			if old[j] == '?' {
				sub[j] = 'b'
				continue next
			}
		}

		return ""
	}

	return string(ans)
}
