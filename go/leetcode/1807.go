package leetcode

func evaluate(s string, knowledge [][]string) string {
	mp := make(map[string]string, len(knowledge))
	for _, kv := range knowledge {
		mp[kv[0]] = kv[1]
	}

	ans := []byte{}
	left := -1

	for i, ch := range s {
		if ch == '(' {
			left = i
		} else if ch == ')' {
			t, ok := mp[s[left+1:i]]
			if !ok {
				t = "?"
			}
			ans = append(ans, t...)
			left = -1
		} else if left < 0 {
			ans = append(ans, byte(ch))
		}
	}

	return string(ans)
}
