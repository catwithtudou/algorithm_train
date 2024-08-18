package leetcode

func checkRecord(s string) bool {
	aNum := 0
	for i := 0; i < len(s); i++ {
		if s[i] == 'A' {
			aNum++
		} else if s[i] == 'L' {
			if i+2 < len(s) && s[i+1] == 'L' && s[i+2] == 'L' {
				return false
			}
		}
		if aNum > 1 {
			return false
		}

	}

	return true
}
