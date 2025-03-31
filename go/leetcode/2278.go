package leetcode

func percentageLetter(s string, letter byte) (ans int) {
	n := len(s)
	cnt := 0
	for _, c := range s {
		if byte(c) == letter {
			cnt++
		}

	}

	return (cnt * 100) / n
}
