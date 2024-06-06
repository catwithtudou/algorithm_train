package leetcode

func minimumSteps(s string) int64 {
	var ans int64
	cnt := 0
	for _, c := range s {
		if c == '1' {
			cnt++
		} else {
			ans += int64(cnt)
		}
	}

	return ans
}
