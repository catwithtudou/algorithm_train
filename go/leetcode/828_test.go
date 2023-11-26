package leetcode

import (
	"fmt"
	"testing"
)

func TestUniqueLetterString(t *testing.T) {
	ans := uniqueLetterString("ABA")
	if ans != 8 {
		fmt.Println(ans)
		t.Fail()
	}

}
