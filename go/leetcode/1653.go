package leetcode

import "strings"

func minimumDeletions1653(s string) int {
	del := strings.Count(s, "a")
	ans := del
	for _, c := range s {
		del += int((c-'a')*2 - 1)
		ans = min(ans, del)
	}
	return ans
}
