package leetcode

import (
	"cmp"
	"strconv"
	"strings"
)

func compareVersion(version1 string, version2 string) int {
	a := strings.Split(version1, ".")
	b := strings.Split(version2, ".")
	n, m := len(a), len(b)
	for i := range max(n, m) {
		ver1 := 0
		if i < n {
			ver1, _ = strconv.Atoi(a[i])
		}
		ver2 := 0
		if i < m {
			ver2, _ = strconv.Atoi(b[i])
		}
		c := cmp.Compare(ver1, ver2)
		if c != 0 {
			return c
		}
	}
	return 0
}
