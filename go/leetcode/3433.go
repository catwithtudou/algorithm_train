package leetcode

import (
	"cmp"
	"strconv"
	"strings"

	"golang.org/x/exp/slices"
)

func countMentions(numberOfUsers int, events [][]string) []int {
	slices.SortFunc(events, func(a, b []string) int {
		ta, _ := strconv.Atoi(a[1])
		tb, _ := strconv.Atoi(b[1])
		return cmp.Or(ta-tb, int(b[0][0])-int(a[0][0]))
	})

	ans := make([]int, numberOfUsers)
	onlineT := make([]int, numberOfUsers)
	for _, e := range events {
		curT, _ := strconv.Atoi(e[1])
		mention := e[2]
		if e[0][0] == 'O' {
			i, _ := strconv.Atoi(mention)
			onlineT[i] = curT + 60
			continue
		}

		if mention[0] == 'A' {
			for i := range ans {
				ans[i]++
			}
		} else if mention[0] == 'H' {
			for i, t := range onlineT {
				if t <= curT {
					ans[i]++
				}
			}
		} else {
			for _, s := range strings.Split(mention, " ") {
				i, _ := strconv.Atoi(s[2:])
				ans[i]++
			}
		}
	}

	return ans
}
