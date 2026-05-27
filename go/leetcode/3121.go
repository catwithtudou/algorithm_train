package leetcode

import "math/bits"

func numberOfSpecialChars3121(word string) int {
	var lower, upper, invalid uint

	for _, c := range word {
		bit := uint(1) << (c & 31)
		if c&32 > 0 {
			lower |= bit
			if upper&bit > 0 {
				invalid |= bit
			}
		} else {
			upper |= bit
		}
	}

	return bits.OnesCount(lower & upper &^ invalid)
}
