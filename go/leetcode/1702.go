package leetcode

import "strings"

func maximumBinaryString(binary string) string {
	i := strings.Index(binary, "0")
	if i < 0 {
		return binary
	}
	cntOne := strings.Count(binary[i:], "1")
	return strings.Repeat("1", len(binary)-1-cntOne) + "0" + strings.Repeat("1", cntOne)
}
