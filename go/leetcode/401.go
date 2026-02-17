package leetcode

import (
	"fmt"
	"math/bits"
)

func readBinaryWatch(turnedOn int) (ans []string) {
	for i := range 12 {
		for j := range 60 {
			if bits.OnesCount(uint(i))+bits.OnesCount(uint(j)) == turnedOn {
				ans = append(ans, fmt.Sprintf("%d:%02d", i, j))
			}
		}
	}
	return
}
