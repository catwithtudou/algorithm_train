package leetcode

func minSwaps(s string) (ans int) {

	c := 0

	for _, b := range s {
		if b == '[' {
			c++
		} else if c > 0 {
			c--
		} else {
			c++
			ans++
		}

	}

	return
}
