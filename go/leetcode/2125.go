package leetcode

import "strings"

func numberOfBeams(bank []string) (ans int) {
	preCnt := 0
	for _, row := range bank {
		cnt := strings.Count(row, "1")
		if cnt > 0 {
			ans += preCnt * cnt
			preCnt = cnt
		}
	}
	return
}
