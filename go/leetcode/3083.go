package leetcode

func isSubstringPresent(s string) bool {
	vis := [26][26]bool{}
	for i := 1; i < len(s); i++ {
		x, y := s[i-1]-'a', s[i]-'a'
		vis[x][y] = true
		if vis[y][x] {
			return true
		}
	}
	return false
}
