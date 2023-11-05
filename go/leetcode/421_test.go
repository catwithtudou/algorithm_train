package leetcode

import "testing"

func TestFindMaximumXOR(t *testing.T) {
	ans := findMaximumXOR([]int{3, 10, 5, 25, 2, 8})
	if ans != 28 {
		t.Fail()
	}
}
