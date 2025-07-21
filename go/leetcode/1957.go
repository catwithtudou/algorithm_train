package leetcode

func makeFancyString(s string) string {
	ans := []byte{}
	cnt := 0
	for i, ch := range s {
		cnt++
		if cnt < 3 {
			ans = append(ans, byte(ch))
		}
		if i < len(s)-1 && byte(ch) != s[i+1] {
			cnt = 0
		}
	}
	return string(ans)
}
