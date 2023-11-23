package leetcode

import "strings"

func entityParser(text string) string {
	entityMap := map[string]string{
		"&quot;":  "\"",
		"&apos;":  "'",
		"&gt;":    ">",
		"&lt;":    "<",
		"&frasl;": "/",
		"&amp;":   "&",
	}

	i := 0
	n := len(text)
	res := make([]string, 0)
	for i < n {
		flag := false
		if text[i] == '&' {
			for k, v := range entityMap {
				if i+len(k) <= n && text[i:i+len(k)] == k {
					res = append(res, v)
					flag = true
					i += len(k)
					break
				}
			}
		}

		if !flag {
			res = append(res, text[i:i+1])
			i++
		}
	}

	return strings.Join(res, "")
}
